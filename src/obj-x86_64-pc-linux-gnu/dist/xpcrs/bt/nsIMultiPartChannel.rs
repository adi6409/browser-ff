//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIMultiPartChannel.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIMultiPartChannel",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute nsIChannel baseChannel; */
                    Method {
                        name: "GetBaseChannel",
                        params: &[Param { name: "aBaseChannel", ty: "*mut*const nsIChannel" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute uint32_t partID; */
                    Method {
                        name: "GetPartID",
                        params: &[Param { name: "aPartID", ty: "*mut uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean isLastPart; */
                    Method {
                        name: "GetIsLastPart",
                        params: &[Param { name: "aIsLastPart", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIMultiPartChannelListener",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onAfterLastPart (in nsresult status); */
                    Method {
                        name: "OnAfterLastPart",
                        params: &[Param { name: "status", ty: "::nserror::nsresult" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

