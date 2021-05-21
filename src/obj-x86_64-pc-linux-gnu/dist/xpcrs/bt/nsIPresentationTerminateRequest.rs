//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/presentation/interfaces/nsIPresentationTerminateRequest.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPresentationTerminateRequest",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute nsIPresentationDevice device; */
                    Method {
                        name: "GetDevice",
                        params: &[Param { name: "aDevice", ty: "*mut*const nsIPresentationDevice" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString presentationId; */
                    Method {
                        name: "GetPresentationId",
                        params: &[Param { name: "aPresentationId", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIPresentationControlChannel controlChannel; */
                    Method {
                        name: "GetControlChannel",
                        params: &[Param { name: "aControlChannel", ty: "*mut*const nsIPresentationControlChannel" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean isFromReceiver; */
                    Method {
                        name: "GetIsFromReceiver",
                        params: &[Param { name: "aIsFromReceiver", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

