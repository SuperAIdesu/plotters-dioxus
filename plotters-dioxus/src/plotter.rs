#![allow(non_snake_case)]

use dioxus::prelude::*;

use plotters_bitmap::BitMapBackend;

use plotters::prelude::*;
use plotters::coord::Shift;

use base64::prelude::*;

use image::ImageEncoder;
use image::codecs::png::PngEncoder;

use std::io::Cursor;

pub type DioxusDrawingArea<'a> = DrawingArea<BitMapBackend<'a>, Shift>;

#[derive(Clone, Props)]
pub struct PlottersProps<F: Fn(DioxusDrawingArea) + Clone + 'static> {
    pub size: (u32, u32),
    pub init: F,
    pub on_click: Option<EventHandler<Event<MouseData>>>,
    pub on_dblclick: Option<EventHandler<Event<MouseData>>>,
    pub on_mousemove: Option<EventHandler<Event<MouseData>>>,
    pub on_mouseout: Option<EventHandler<Event<MouseData>>>,
    pub on_mouseup: Option<EventHandler<Event<MouseData>>>,
    pub on_mousedown: Option<EventHandler<Event<MouseData>>>,
    pub on_mouseover: Option<EventHandler<Event<MouseData>>>,
    pub on_wheel: Option<EventHandler<Event<WheelData>>>,
    #[props(default = false)]
    pub draggable: bool,
    pub on_drag: Option<EventHandler<Event<DragData>>>,
    pub on_dragend: Option<EventHandler<Event<DragData>>>,
    pub on_dragenter: Option<EventHandler<Event<DragData>>>,
    pub on_dragleave: Option<EventHandler<Event<DragData>>>,
    pub on_dragover: Option<EventHandler<Event<DragData>>>,
    pub on_dragstart: Option<EventHandler<Event<DragData>>>,
    pub on_drop: Option<EventHandler<Event<DragData>>>,
    pub on_scroll: Option<EventHandler<Event<ScrollData>>>,
}

impl<F: Fn(DioxusDrawingArea) + Clone + 'static> PartialEq for PlottersProps<F> {
    fn eq(&self, other: &PlottersProps<F>) -> bool {
        self.size == other.size
    }
}

pub fn Plotters<F: Fn(DioxusDrawingArea) + Clone + 'static>(props: PlottersProps<F>) -> Element {
    let buffer_size = ((props.size.1 * props.size.0) as usize) * 3usize;
    let mut buffer = vec![0u8; buffer_size];
    let drawing_area = BitMapBackend::with_buffer(
        buffer.as_mut_slice(),
        props.size
    ).into_drawing_area();
    (props.init)(drawing_area);

    let mut data = vec![0; 0];
    let cursor = Cursor::new(&mut data);
    let encoder = PngEncoder::new(cursor);
    let color = image::ExtendedColorType::Rgb8;

    encoder
        .write_image(buffer.as_slice(), props.size.0, props.size.1, color)
        .expect("The Png encoder is expected to write the image");

    let buffer_base64 = BASE64_STANDARD.encode(data);

    rsx!(img {
        onclick: move |e| {
            if let Some(callback) = props.on_click.as_ref() {
                callback.call(e)
            };
        },
        ondoubleclick: move |e| {
            if let Some(callback) = props.on_dblclick.as_ref() {
                callback.call(e)
            };
        },
        onmousemove: move |e| {
            if let Some(callback) = props.on_mousemove.as_ref() {
                callback.call(e)
            };
        },
        onmousedown: move |e| {
            if let Some(callback) = props.on_mousedown.as_ref() {
                callback.call(e)
            };
        },
        onmouseup: move |e| {
            if let Some(callback) = props.on_mouseup.as_ref() {
                callback.call(e)
            };
        },
        onmouseout: move |e| {
            if let Some(callback) = props.on_mouseout.as_ref() {
                callback.call(e)
            };
        },
        onmouseover: move |e| {
            if let Some(callback) = props.on_mouseover.as_ref() {
                callback.call(e)
            };
        },
        onwheel: move |e| {
            if let Some(callback) = props.on_wheel.as_ref() {
                callback.call(e)
            };
        },
        draggable: props.draggable,
        ondrag: move |e| {
            if let Some(callback) = props.on_drag.as_ref() {
                callback.call(e)
            };
        },
        ondragend: move |e| {
            if let Some(callback) = props.on_dragend.as_ref() {
                callback.call(e)
            };
        },
        ondragenter: move |e| {
            if let Some(callback) = props.on_dragenter.as_ref() {
                callback.call(e)
            };
        },
        ondragleave: move |e| {
            if let Some(callback) = props.on_dragleave.as_ref() {
                callback.call(e)
            };
        },
        ondragover: move |e| {
            if let Some(callback) = props.on_dragover.as_ref() {
                callback.call(e)
            };
        },
        ondragstart: move |e| {
            if let Some(callback) = props.on_dragstart.as_ref() {
                callback.call(e)
            };
        },
        ondrop: move |e| {
            if let Some(callback) = props.on_drop.as_ref() {
                callback.call(e)
            };
        },
        onscroll: move |e| {
            if let Some(callback) = props.on_scroll.as_ref() {
                callback.call(e)
            };
        },
        src: "data:image/png;base64,{buffer_base64}",
    })
}