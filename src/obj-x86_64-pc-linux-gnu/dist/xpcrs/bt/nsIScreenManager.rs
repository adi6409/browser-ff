//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsIScreenManager.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIScreenManager",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* nsIScreen screenForRect (in long left, in long top, in long width, in long height); */
                    Method {
                        name: "ScreenForRect",
                        params: &[Param { name: "left", ty: "i32" }, Param { name: "top", ty: "i32" }, Param { name: "width", ty: "i32" }, Param { name: "height", ty: "i32" }, Param { name: "_retval", ty: "*mut *const nsIScreen" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIScreen primaryScreen; */
                    Method {
                        name: "GetPrimaryScreen",
                        params: &[Param { name: "aPrimaryScreen", ty: "*mut *const nsIScreen" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute int64_t totalScreenPixels; */
                    Method {
                        name: "GetTotalScreenPixels",
                        params: &[Param { name: "aTotalScreenPixels", ty: "*mut int64_t" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

