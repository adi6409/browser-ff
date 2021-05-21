//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessibleImage.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAccessibleImage",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void getImagePosition (in unsigned long coordType, out long x, out long y); */
                    Method {
                        name: "GetImagePosition",
                        params: &[Param { name: "coordType", ty: "u32" }, Param { name: "x", ty: "*mut i32" }, Param { name: "y", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void getImageSize (out long width, out long height); */
                    Method {
                        name: "GetImageSize",
                        params: &[Param { name: "width", ty: "*mut i32" }, Param { name: "height", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

