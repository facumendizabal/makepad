#![cfg_attr(all(unix, use_unstable_unix_socket_ancillary_data_2021), feature(unix_socket_ancillary_data))]

pub mod os;

#[macro_use]
mod live_prims;

#[macro_use]
mod cx;
mod cx_api;

pub mod live_traits;
pub mod live_cx;
pub mod live_atomic;

pub mod thread;
pub mod audio;
pub mod midi;
pub mod video;

mod draw_matrix;
mod draw_shader; 
mod draw_list;
mod draw_vars;

mod id_pool;
pub mod event;
mod area;
mod window;
mod pass;
mod texture;
mod cursor;
mod macos_menu;
mod animator;
mod gpu_info;
mod geometry;
mod debug;
mod component_map;
mod performance_stats;

pub mod audio_stream;

mod media_api;
mod decoding_api;

#[macro_use]
mod app_main;

#[cfg(target_arch = "wasm32")]
pub use makepad_wasm_bridge;

#[cfg(any(target_os = "macos", target_os = "ios"))]
pub use makepad_objc_sys;

#[cfg(target_os = "windows")]
pub use ::makepad_windows as windows;

pub use makepad_futures;
 
pub use {
    makepad_shader_compiler,
    makepad_shader_compiler::makepad_derive_live,
    makepad_shader_compiler::makepad_math,
    makepad_shader_compiler::makepad_live_tokenizer,
    makepad_shader_compiler::makepad_micro_serde,
    makepad_shader_compiler::makepad_live_compiler,
    makepad_shader_compiler::makepad_live_id,
    makepad_shader_compiler::makepad_error_log,
    //makepad_image_formats::image,
    makepad_derive_live::*,
    makepad_error_log::*,
    makepad_math::*,
    makepad_live_id::*,
    app_main::AppMain,
    makepad_live_compiler::{
        vec4_ext::*,
        live_error_origin,
        live_eval,
        LiveEval,
        LiveErrorOrigin,
        LiveNodeOrigin,
        LiveRegistry,
        LiveId,
        LiveIdMap,
        LiveFileId,
        LivePtr,
        LiveRef,
        LiveNode,
        LiveType,
        LiveTypeInfo,
        LiveTypeField,
        LiveFieldKind,
        LiveComponentInfo,
        LiveComponentRegistry,
        LivePropType,
        LiveProp,
        LiveIdAsProp,
        LiveValue,
        InlineString,
        LiveBinding,
        LiveIdPath,
        LiveNodeSliceToCbor,
        LiveNodeVecFromCbor,
        LiveModuleId,
        LiveNodeSlice,
        LiveNodeVec,
        LiveNodeSliceApi,
        LiveNodeVecApi,
    },
    component_map::ComponentMap,
    makepad_shader_compiler::{
        ShaderRegistry,
        ShaderEnum,
        DrawShaderPtr,
        ShaderTy,
    },
    crate::{
        os::*,
        decoding_api::CxDecodingApi,
        cx_api::CxOsApi,
        media_api::CxMediaApi,
        draw_list::{
            CxDrawItem,
            CxRectArea,
            CxDrawCall,
            DrawList,
            DrawListId,
            CxDrawListPool
        },
        cx::{
            Cx,
            CxRef,
            OsType
        },
        area::{
            Area,
            RectArea,
            InstanceArea
        },
        midi::*,
        audio::*,
        thread::*,
        video::*,
        event::{
            VirtualKeyboardEvent,
            HttpRequest,
            HttpResponse,
            HttpMethod,
            NetworkResponse,
            NetworkResponseEvent,
            Margin,
            KeyCode,
            Event,
            Hit,
            DragHit,
            Trigger,
            //MidiInputListEvent,
            Timer,
            NextFrame,
            KeyModifiers,
            DrawEvent,
            DigitDevice,
            MouseDownEvent,
            MouseMoveEvent,
            MouseUpEvent,
            FingerDownEvent,
            FingerMoveEvent,
            FingerUpEvent,
            HoverState,
            FingerHoverEvent,
            FingerScrollEvent,
            WindowGeomChangeEvent,
            WindowMovedEvent,
            NextFrameEvent,
            TimerEvent,
            KeyEvent,
            KeyFocusEvent,
            TextInputEvent,
            TextClipboardEvent,
            WindowCloseRequestedEvent,
            WindowClosedEvent,
            WindowDragQueryResponse,
            WindowDragQueryEvent,
            XRButton,
            XRInput,
            XRUpdateEvent,
            DragEvent,
            DropEvent,
            DragState,
            DragItem,
            DragResponse,
            HitOptions,
            DragHitEvent,
            DropHitEvent,
            VideoColorFormat,
        },
        cursor::MouseCursor,
        macos_menu::MacosMenu,
        draw_matrix::DrawMatrix,
        window::WindowHandle,
        pass::{
            PassId,
            CxPassParent,
            CxPassRect,
            Pass,
            PassClearColor,
            PassClearDepth
        },
        texture::{
            Texture,
            TextureId,
            TextureFormat,
            TextureDesc
        },
        live_prims::{
            LiveDependency,
            RcStringMut,
        },
        live_traits::{
            LiveHookDeref,
            LiveBody,
            LiveNew,
            LiveApply,
            LiveHook,
            LiveApplyValue,
            LiveRead,
            ToLiveValue,
            ApplyFrom,
        },
        animator::{
            Ease,
            Play,
            Animate,
            Animator,
            AnimatorImpl,
            AnimatorAction,
        },
        draw_vars::{
            shader_enum,
            DrawVars
        },
        geometry::{
            GeometryFingerprint,
            GeometryField,
            GeometryFields,
            GeometryId,
            GeometryRef,
            Geometry,
        },
        gpu_info::GpuPerformance,       
    },
};

