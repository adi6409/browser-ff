//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsIDirectoryService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDirectoryServiceProvider",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* nsIFile getFile (in string prop, out boolean persistent); */
                    Method {
                        name: "GetFile",
                        params: &[Param { name: "prop", ty: "*const libc::c_char" }, Param { name: "persistent", ty: "*mut bool" }, Param { name: "_retval", ty: "*mut*const nsIFile" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIDirectoryServiceProvider2",
            base: Some("nsIDirectoryServiceProvider"),
            methods: Ok(&[
                    /* nsISimpleEnumerator getFiles (in string prop); */
                    Method {
                        name: "GetFiles",
                        params: &[Param { name: "prop", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut*const nsISimpleEnumerator" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIDirectoryService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void init (); */
                    Method {
                        name: "Init",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void registerProvider (in nsIDirectoryServiceProvider prov); */
                    Method {
                        name: "RegisterProvider",
                        params: &[Param { name: "prov", ty: "*const nsIDirectoryServiceProvider" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void unregisterProvider (in nsIDirectoryServiceProvider prov); */
                    Method {
                        name: "UnregisterProvider",
                        params: &[Param { name: "prov", ty: "*const nsIDirectoryServiceProvider" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

