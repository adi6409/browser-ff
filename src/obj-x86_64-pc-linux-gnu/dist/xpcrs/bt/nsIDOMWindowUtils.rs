//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/base/nsIDOMWindowUtils.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMWindowUtils",
            base: Some("nsISupports"),
            methods: Err("optional_argc is unsupported"),
        },

        Interface {
            name: "nsITranslationNodeList",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute unsigned long length; */
                    Method {
                        name: "GetLength",
                        params: &[Param { name: "aLength", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* Node item (in unsigned long index); */
                    Method {
                        name: "Item",
                        params: &[Param { name: "index", ty: "u32" }, Param { name: "_retval", ty: "*mut *const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean isTranslationRootAtIndex (in unsigned long index); */
                    Method {
                        name: "IsTranslationRootAtIndex",
                        params: &[Param { name: "index", ty: "u32" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIJSRAIIHelper",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void destruct (); */
                    Method {
                        name: "Destruct",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

