//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/xul/nsIDOMXULRelatedElement.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMXULRelatedElement",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* Element getRelatedElement (in Node aElement); */
                    Method {
                        name: "GetRelatedElement",
                        params: &[Param { name: "aElement", ty: "*const libc::c_void" }, Param { name: "_retval", ty: "*mut *const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

