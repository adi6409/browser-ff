//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/payments/nsIPaymentUIService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPaymentUIService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void showPayment (in AString requestId); */
                    Method {
                        name: "ShowPayment",
                        params: &[Param { name: "requestId", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void abortPayment (in AString requestId); */
                    Method {
                        name: "AbortPayment",
                        params: &[Param { name: "requestId", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void completePayment (in AString requestId); */
                    Method {
                        name: "CompletePayment",
                        params: &[Param { name: "requestId", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void updatePayment (in AString requestId); */
                    Method {
                        name: "UpdatePayment",
                        params: &[Param { name: "requestId", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void closePayment (in AString requestId); */
                    Method {
                        name: "ClosePayment",
                        params: &[Param { name: "requestId", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

