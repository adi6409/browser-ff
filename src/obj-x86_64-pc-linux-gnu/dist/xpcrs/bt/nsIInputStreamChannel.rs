//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIInputStreamChannel.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIInputStreamChannel",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void setURI (in nsIURI aURI); */
                    Method {
                        name: "SetURI",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute nsIInputStream contentStream; */
                    Method {
                        name: "GetContentStream",
                        params: &[Param { name: "aContentStream", ty: "*mut*const nsIInputStream" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetContentStream",
                        params: &[Param { name: "aContentStream", ty: "*const nsIInputStream" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute AString srcdocData; */
                    Method {
                        name: "GetSrcdocData",
                        params: &[Param { name: "aSrcdocData", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetSrcdocData",
                        params: &[Param { name: "aSrcdocData", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean isSrcdocChannel; */
                    Method {
                        name: "GetIsSrcdocChannel",
                        params: &[Param { name: "aIsSrcdocChannel", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute nsIURI baseURI; */
                    Method {
                        name: "GetBaseURI",
                        params: &[Param { name: "aBaseURI", ty: "*mut*const nsIURI" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetBaseURI",
                        params: &[Param { name: "aBaseURI", ty: "*const nsIURI" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

