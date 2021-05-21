//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/indexedDB/nsIIDBPermissionsRequest.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIIDBPermissionsRequest",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute Element browserElement; */
                    Method {
                        name: "GetBrowserElement",
                        params: &[Param { name: "aBrowserElement", ty: "*mut *const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIObserver responseObserver; */
                    Method {
                        name: "GetResponseObserver",
                        params: &[Param { name: "aResponseObserver", ty: "*mut*const nsIObserver" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

