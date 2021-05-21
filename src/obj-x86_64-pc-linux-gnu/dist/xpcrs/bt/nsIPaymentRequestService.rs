//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/payments/nsIPaymentRequestService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPaymentRequestService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* nsIPaymentRequest getPaymentRequestById (in AString aRequestId); */
                    Method {
                        name: "GetPaymentRequestById",
                        params: &[Param { name: "aRequestId", ty: "*const ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut *const nsIPaymentRequest" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsISimpleEnumerator enumerate (); */
                    Method {
                        name: "Enumerate",
                        params: &[Param { name: "_retval", ty: "*mut *const nsISimpleEnumerator" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void respondPayment (in nsIPaymentActionResponse aResponse); */
                    Method {
                        name: "RespondPayment",
                        params: &[Param { name: "aResponse", ty: "*const nsIPaymentActionResponse" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void changeShippingAddress (in AString requestId, in nsIPaymentAddress aAddress); */
                    Method {
                        name: "ChangeShippingAddress",
                        params: &[Param { name: "requestId", ty: "*const ::nsstring::nsAString" }, Param { name: "aAddress", ty: "*const nsIPaymentAddress" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void changeShippingOption (in AString requestId, in AString option); */
                    Method {
                        name: "ChangeShippingOption",
                        params: &[Param { name: "requestId", ty: "*const ::nsstring::nsAString" }, Param { name: "option", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void changePayerDetail (in AString requestId, in AString aPayerName, in AString aPayerEmail, in AString aPayerPhone); */
                    Method {
                        name: "ChangePayerDetail",
                        params: &[Param { name: "requestId", ty: "*const ::nsstring::nsAString" }, Param { name: "aPayerName", ty: "*const ::nsstring::nsAString" }, Param { name: "aPayerEmail", ty: "*const ::nsstring::nsAString" }, Param { name: "aPayerPhone", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void changePaymentMethod (in AString requestId, in AString aMethodName, in nsIMethodChangeDetails aMethodDetails); */
                    Method {
                        name: "ChangePaymentMethod",
                        params: &[Param { name: "requestId", ty: "*const ::nsstring::nsAString" }, Param { name: "aMethodName", ty: "*const ::nsstring::nsAString" }, Param { name: "aMethodDetails", ty: "*const nsIMethodChangeDetails" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void cleanup (); */
                    Method {
                        name: "Cleanup",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void setTestingUIService (in nsIPaymentUIService aUIService); */
                    Method {
                        name: "SetTestingUIService",
                        params: &[Param { name: "aUIService", ty: "*const nsIPaymentUIService" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

