//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/browser/nsIEmbeddingSiteWindow.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIEmbeddingSiteWindow",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void setDimensions (in unsigned long flags, in long x, in long y, in long cx, in long cy); */
                    Method {
                        name: "SetDimensions",
                        params: &[Param { name: "flags", ty: "u32" }, Param { name: "x", ty: "i32" }, Param { name: "y", ty: "i32" }, Param { name: "cx", ty: "i32" }, Param { name: "cy", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void getDimensions (in unsigned long flags, out long x, out long y, out long cx, out long cy); */
                    Method {
                        name: "GetDimensions",
                        params: &[Param { name: "flags", ty: "u32" }, Param { name: "x", ty: "*mut i32" }, Param { name: "y", ty: "*mut i32" }, Param { name: "cx", ty: "*mut i32" }, Param { name: "cy", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setFocus (); */
                    Method {
                        name: "SetFocus",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute boolean visibility; */
                    Method {
                        name: "GetVisibility",
                        params: &[Param { name: "aVisibility", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetVisibility",
                        params: &[Param { name: "aVisibility", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute AString title; */
                    Method {
                        name: "GetTitle",
                        params: &[Param { name: "aTitle", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetTitle",
                        params: &[Param { name: "aTitle", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [noscript] readonly attribute voidPtr siteWindow; */
                    Method {
                        name: "GetSiteWindow",
                        params: &[Param { name: "aSiteWindow", ty: "*mut *mut libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void blur (); */
                    Method {
                        name: "Blur",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

