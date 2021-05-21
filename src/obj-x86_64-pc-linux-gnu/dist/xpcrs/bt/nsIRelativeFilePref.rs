//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/modules/libpref/nsIRelativeFilePref.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIRelativeFilePref",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* attribute nsIFile file; */
                    Method {
                        name: "GetFile",
                        params: &[Param { name: "aFile", ty: "*mut*const nsIFile" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetFile",
                        params: &[Param { name: "aFile", ty: "*const nsIFile" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute ACString relativeToKey; */
                    Method {
                        name: "GetRelativeToKey",
                        params: &[Param { name: "aRelativeToKey", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetRelativeToKey",
                        params: &[Param { name: "aRelativeToKey", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

