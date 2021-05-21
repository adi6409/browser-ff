//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIFileURL.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIFileURL",
            base: Some("nsIURL"),
            methods: Ok(&[
                    /* readonly attribute nsIFile file; */
                    Method {
                        name: "GetFile",
                        params: &[Param { name: "aFile", ty: "*mut*const nsIFile" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIFileURLMutator",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* [must_use,noscript] void markFileURL (); */
                    Method {
                        name: "MarkFileURL",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use,noscript] void setFile (in nsIFile aFile); */
                    Method {
                        name: "SetFile",
                        params: &[Param { name: "aFile", ty: "*const nsIFile" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

