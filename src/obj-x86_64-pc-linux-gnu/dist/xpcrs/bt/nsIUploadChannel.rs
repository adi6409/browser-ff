//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIUploadChannel.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIUploadChannel",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void setUploadStream (in nsIInputStream aStream, in ACString aContentType, in long long aContentLength); */
                    Method {
                        name: "SetUploadStream",
                        params: &[Param { name: "aStream", ty: "*const nsIInputStream" }, Param { name: "aContentType", ty: "*const ::nsstring::nsACString" }, Param { name: "aContentLength", ty: "i64" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIInputStream uploadStream; */
                    Method {
                        name: "GetUploadStream",
                        params: &[Param { name: "aUploadStream", ty: "*mut*const nsIInputStream" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

