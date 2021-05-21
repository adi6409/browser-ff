//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIURL.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIURL",
            base: Some("nsIURI"),
            methods: Ok(&[
                    /* readonly attribute AUTF8String directory; */
                    Method {
                        name: "GetDirectory",
                        params: &[Param { name: "aDirectory", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AUTF8String fileName; */
                    Method {
                        name: "GetFileName",
                        params: &[Param { name: "aFileName", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AUTF8String fileBaseName; */
                    Method {
                        name: "GetFileBaseName",
                        params: &[Param { name: "aFileBaseName", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AUTF8String fileExtension; */
                    Method {
                        name: "GetFileExtension",
                        params: &[Param { name: "aFileExtension", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AUTF8String getCommonBaseSpec (in nsIURI aURIToCompare); */
                    Method {
                        name: "GetCommonBaseSpec",
                        params: &[Param { name: "aURIToCompare", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AUTF8String getRelativeSpec (in nsIURI aURIToCompare); */
                    Method {
                        name: "GetRelativeSpec",
                        params: &[Param { name: "aURIToCompare", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIURLMutator",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* [must_use] nsIURIMutator setFileName (in AUTF8String aFileName); */
                    Method {
                        name: "SetFileName",
                        params: &[Param { name: "aFileName", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut*const nsIURIMutator" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] nsIURIMutator setFileBaseName (in AUTF8String aFileBaseName); */
                    Method {
                        name: "SetFileBaseName",
                        params: &[Param { name: "aFileBaseName", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut*const nsIURIMutator" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] nsIURIMutator setFileExtension (in AUTF8String aFileExtension); */
                    Method {
                        name: "SetFileExtension",
                        params: &[Param { name: "aFileExtension", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut*const nsIURIMutator" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

