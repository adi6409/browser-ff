//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsIScreen.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIScreen",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void GetRect (out long left, out long top, out long width, out long height); */
                    Method {
                        name: "GetRect",
                        params: &[Param { name: "left", ty: "*mut i32" }, Param { name: "top", ty: "*mut i32" }, Param { name: "width", ty: "*mut i32" }, Param { name: "height", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void GetAvailRect (out long left, out long top, out long width, out long height); */
                    Method {
                        name: "GetAvailRect",
                        params: &[Param { name: "left", ty: "*mut i32" }, Param { name: "top", ty: "*mut i32" }, Param { name: "width", ty: "*mut i32" }, Param { name: "height", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void GetRectDisplayPix (out long left, out long top, out long width, out long height); */
                    Method {
                        name: "GetRectDisplayPix",
                        params: &[Param { name: "left", ty: "*mut i32" }, Param { name: "top", ty: "*mut i32" }, Param { name: "width", ty: "*mut i32" }, Param { name: "height", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void GetAvailRectDisplayPix (out long left, out long top, out long width, out long height); */
                    Method {
                        name: "GetAvailRectDisplayPix",
                        params: &[Param { name: "left", ty: "*mut i32" }, Param { name: "top", ty: "*mut i32" }, Param { name: "width", ty: "*mut i32" }, Param { name: "height", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long pixelDepth; */
                    Method {
                        name: "GetPixelDepth",
                        params: &[Param { name: "aPixelDepth", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long colorDepth; */
                    Method {
                        name: "GetColorDepth",
                        params: &[Param { name: "aColorDepth", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute double contentsScaleFactor; */
                    Method {
                        name: "GetContentsScaleFactor",
                        params: &[Param { name: "aContentsScaleFactor", ty: "*mut libc::c_double" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute double defaultCSSScaleFactor; */
                    Method {
                        name: "GetDefaultCSSScaleFactor",
                        params: &[Param { name: "aDefaultCSSScaleFactor", ty: "*mut libc::c_double" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute float dpi; */
                    Method {
                        name: "GetDpi",
                        params: &[Param { name: "aDpi", ty: "*mut libc::c_float" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

