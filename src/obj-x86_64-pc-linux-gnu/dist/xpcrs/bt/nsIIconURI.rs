//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/image/nsIIconURI.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIMozIconURI",
            base: Some("nsIURI"),
            methods: Ok(&[
                    /* readonly attribute nsIURL iconURL; */
                    Method {
                        name: "GetIconURL",
                        params: &[Param { name: "aIconURL", ty: "*mut *const nsIURL" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long imageSize; */
                    Method {
                        name: "GetImageSize",
                        params: &[Param { name: "aImageSize", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute ACString stockIcon; */
                    Method {
                        name: "GetStockIcon",
                        params: &[Param { name: "aStockIcon", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute ACString iconSize; */
                    Method {
                        name: "GetIconSize",
                        params: &[Param { name: "aIconSize", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute ACString iconState; */
                    Method {
                        name: "GetIconState",
                        params: &[Param { name: "aIconState", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute ACString contentType; */
                    Method {
                        name: "GetContentType",
                        params: &[Param { name: "aContentType", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute ACString fileExtension; */
                    Method {
                        name: "GetFileExtension",
                        params: &[Param { name: "aFileExtension", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

