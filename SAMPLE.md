## Detailed sample:

Let's try sorting the declarations in an [older checkout of `gfx/lib.rs` module](https://github.com/servo/servo/blob/8f1469eb08a437bcc6cfb510334be2b6430b4a8f/components/gfx/lib.rs) in Servo (I chose this file specifically, because it covers all the use cases). Once we add the plugin to the dependencies, we get some pretty warnings.

Let's see them one by one...

### `extern crate`

So, we have this first set of mess in `gfx/lib.rs`...

``` rust
#[macro_use]
extern crate log;
extern crate serde;

extern crate azure;
#[macro_use] extern crate bitflags;
extern crate fnv;
extern crate euclid;
extern crate ipc_channel;
#[macro_use]
extern crate lazy_static;
extern crate layers;
extern crate libc;
#[macro_use]
extern crate profile_traits;
extern crate script_traits;
extern crate rustc_serialize;
extern crate net_traits;
#[macro_use]
extern crate util;
extern crate msg;
extern crate rand;

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
extern crate simd;

extern crate smallvec;
extern crate string_cache;
extern crate style;
extern crate skia;
extern crate time;
extern crate url;

extern crate gfx_traits;
extern crate canvas_traits;

// Eventually we would like the shaper to be pluggable, as many operating systems have their own
// shapers. For now, however, this is a hard dependency.
extern crate harfbuzz;

// Linux and Android-specific library dependencies
#[cfg(any(target_os = "linux", target_os = "android"))]
extern crate fontconfig;

#[cfg(any(target_os = "linux", target_os = "android"))]
extern crate freetype;

// Mac OS-specific library dependencies
#[cfg(target_os = "macos")] extern crate core_foundation;
#[cfg(target_os = "macos")] extern crate core_graphics;
#[cfg(target_os = "macos")] extern crate core_text;
```

... for which we get the following warning,

    gfx/lib.rs:26:1: 70:23 warning: crate declarations should be in alphabetical order!
    Try this...

    #[macro_use]
    extern crate bitflags;
    #[macro_use]
    extern crate lazy_static;
    #[macro_use]
    extern crate log;
    #[macro_use]
    extern crate profile_traits;
    #[macro_use]
    extern crate util;
    extern crate azure;
    extern crate canvas_traits;
    extern crate euclid;
    extern crate fnv;
    #[cfg(any(target_os = "linux", target_os = "android"))]
    extern crate fontconfig;
    #[cfg(any(target_os = "linux", target_os = "android"))]
    extern crate freetype;
    extern crate gfx_traits;
    extern crate harfbuzz;
    extern crate ipc_channel;
    extern crate layers;
    extern crate libc;
    extern crate msg;
    extern crate net_traits;
    extern crate rand;
    extern crate rustc_serialize;
    extern crate script_traits;
    extern crate serde;
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
    extern crate simd;
    extern crate skia;
    extern crate smallvec;
    extern crate string_cache;
    extern crate style;
    extern crate time;
    extern crate url;

    , #[warn(unsorted_declarations)] on by default
    gfx/lib.rs:26 extern crate log;
    gfx/lib.rs:27 extern crate serde;
    gfx/lib.rs:28
    gfx/lib.rs:29 extern crate azure;we can't
                                                      ...

As you can see, **sorty is blind to spaces and comments** (for now). You should always remember that it's a plugin. All it does is play with the AST and rebuild the input back from there. So, the output is as expected - stuff with `#[macro_use]` are sorted and moved to the top, while those with other attributes are below them.

It shows exactly where you should replace the code,

    gfx/lib.rs:26:1: 70:23 warning: ...

... which is useful when (for example) half of the code's already sorted.

Also, note that the declarations with attributes like `#[cfg(target_os = "macos")]` are stripped off while building in linux (for example), and so the lint can't see the `core_foundation`, `core_graphics` and `core_text` crates in my system, because the stripping happens well before lints. So, folks should take care of that.

### `mod`

Now, off to the second set of mess. In `gfx/lib.rs`, we have...

``` rust
// Private painting modules
mod paint_context;

#[path = "display_list/mod.rs"]
pub mod display_list;
pub mod paint_task;

// Fonts
pub mod font;
pub mod font_cache_task;
pub mod font_context;
pub mod font_template;

// Misc.
mod filters;

// Platform-specific implementations.
#[path = "platform/mod.rs"]
pub mod platform;

// Text
#[path = "text/mod.rs"]
pub mod text;
```

and, we get...

    gfx/lib.rs:80:1: 101:14 warning: module declarations (other than inline modules)
    should be in alphabetical order!
    Try this...

    mod filters;
    mod paint_context;
    #[deny(unsafe_code)]
    #[path = "display_list/mod.rs"]
    pub mod display_list;
    pub mod font;
    pub mod font_cache_task;
    pub mod font_context;
    pub mod font_template;
    pub mod paint_task;
    #[path = "platform/mod.rs"]
    pub mod platform;
    #[path = "text/mod.rs"]
    pub mod text;

    , #[warn(unsorted_declarations)] on by default
    gfx/lib.rs:80 mod paint_context;
    gfx/lib.rs:81
    gfx/lib.rs:82 #[path = "display_list/mod.rs"]
    gfx/lib.rs:83 pub mod display_list;
    gfx/lib.rs:84 pub mod paint_task;
    gfx/lib.rs:85
                                                          ...

The comments and spaces are ignored, just like I'd said previously. And, the private modules are sorted and moved to the top, while the public modules are sorted and moved to the bottom. If there were any `#[macro_use]`, it would've been moved to the top irrespective of whether it's public or private.

Anyways, there's more. The public module `display_list` only had a `path` attribute, but we now have a `#[deny(unsafe_code)]` along with it. That's because the `display_list` [has the attribute in its file](https://github.com/servo/servo/blob/8f1469eb08a437bcc6cfb510334be2b6430b4a8f/components/gfx/display_list/mod.rs#L17). I'm not sure whether it's a bad style, but even if it's good, there's no way we could train the lint to detect them, because in the AST level, there's no difference between both the cases i.e., it doesn't matter where you declare the attribute.

So, that should be taken care of as well...

### `use`

There's only one `use` statement in `gfx/lib.rs`, which sorty doesn't mind. But, since it makes use of `check_mod` function, `rustc` checks all the modules, by which I mean that it walks into the modules and submodules (and subsubmodules and ...) of the crate, and so we have warnings for the `use` statements declared there. Let's choose [`gfx/paint_task.rs`](https://github.com/servo/servo/blob/8f1469eb08a437bcc6cfb510334be2b6430b4a8f/components/gfx/paint_task.rs), which has the following contents...

``` rust
use azure::AzFloat;
use azure::azure_hl::{SurfaceFormat, Color, DrawTarget, BackendType};
use canvas_traits::CanvasMsg;
use display_list::{self, StackingContext};
use euclid::Matrix4;
use euclid::point::Point2D;
use euclid::rect::Rect;
use euclid::size::Size2D;
use font_cache_task::FontCacheTask;
use font_context::FontContext;
use ipc_channel::ipc::IpcSender;
use layers::layers::{BufferRequest, LayerBuffer, LayerBufferSet};
use layers::platform::surface::{NativeDisplay, NativeSurface};
use msg::compositor_msg::{Epoch, FrameTreeId, LayerId, LayerKind};
use msg::compositor_msg::{LayerProperties, PaintListener};
use msg::constellation_msg::Msg as ConstellationMsg;
use msg::constellation_msg::PipelineExitType;
use msg::constellation_msg::{ConstellationChan, Failure, PipelineId};
use paint_context::PaintContext;
use profile_traits::mem::{self, ReportsChan};
use profile_traits::time::{self, profile};
use rand::{self, Rng};
use skia::gl_context::GLContext;
use std::borrow::ToOwned;
use std::collections::HashMap;
use std::mem as std_mem;
use std::sync::Arc;
use std::sync::mpsc::{Receiver, Select, Sender, channel};
use url::Url;
use util::geometry::{Au, ZERO_POINT};
use util::opts;
use util::task::spawn_named;
use util::task::spawn_named_with_send_on_failure;
use util::task_state;
```

and, we get the following warning...

    /gfx/paint_task.rs:8:5: 40:22 warning: use statements should be in alphabetical order!
    Try this...

    use azure::azure_hl::{BackendType, Color, DrawTarget, SurfaceFormat};
    use canvas_traits::CanvasMsg;
    use display_list::{self, StackingContext};
    use euclid::Matrix4;
    use euclid::point::Point2D;
    use euclid::rect::Rect;
    use euclid::size::Size2D;
    use font_cache_task::FontCacheTask;
    use font_context::FontContext;
    use ipc_channel::ipc::IpcSender;
    use layers::layers::{BufferRequest, LayerBuffer, LayerBufferSet};
    use layers::platform::surface::{NativeDisplay, NativeSurface};
    use msg::compositor_msg::{Epoch, FrameTreeId, LayerId, LayerKind};
    use msg::compositor_msg::{LayerProperties, PaintListener};
    use msg::constellation_msg::Msg as ConstellationMsg;
    use msg::constellation_msg::PipelineExitType;
    use msg::constellation_msg::{ConstellationChan, Failure, PipelineId};
    use paint_context::PaintContext;
    use profile_traits::mem::{self, ReportsChan};
    use profile_traits::time::{self, profile};
    use rand::{self, Rng};
    use skia::gl_context::GLContext;
    use std::borrow::ToOwned;
    use std::collections::HashMap;
    use std::mem as std_mem;
    use std::sync::Arc;
    use std::sync::mpsc::{Receiver, Select, Sender, channel};
    use url::Url;
    use util::geometry::{Au, ZERO_POINT};
    use util::opts;
    use util::task::spawn_named;
    use util::task::spawn_named_with_send_on_failure;
    use util::task_state;

    , #[warn(unsorted_declarations)] on by default
    gfx/paint_task.rs: 8 use azure::azure_hl::{SurfaceFormat, Color, DrawTarget, BackendType};
    gfx/paint_task.rs: 9 use canvas_traits::CanvasMsg;
    gfx/paint_task.rs:10 use display_list::{self, StackingContext};
    gfx/paint_task.rs:11 use euclid::Matrix4;
    gfx/paint_task.rs:12 use euclid::point::Point2D;
    gfx/paint_task.rs:13 use euclid::rect::Rect;
                                                                 ...

As you can see, the first declaration `use azure::AzFloat;` has been left out, and the span has began from line 8 instead of line 7. This means that in the eyes of sorty, the first declaration stays right where it was even after sorting, but the statements following it aren't following the rule (though the second line stays where it was, its list items aren't sorted!), and so it throws the warning on all of the remaining statements.

Also, note that sorty doesn't care about the other statements. If one statement's wrong, then it starts printing out all the following statements, because it's easier for everyone to just copy-paste everything instead of looking for the exact span of code.
