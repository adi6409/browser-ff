//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/payments/nsIPaymentRequest.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPaymentMethodData",
            base: Some("nsISupports"),
            methods: Err("specialtype jsval unsupported"),
        },

        Interface {
            name: "nsIPaymentCurrencyAmount",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute AString currency; */
                    Method {
                        name: "GetCurrency",
                        params: &[Param { name: "aCurrency", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString value; */
                    Method {
                        name: "GetValue",
                        params: &[Param { name: "aValue", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIPaymentItem",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute AString label; */
                    Method {
                        name: "GetLabel",
                        params: &[Param { name: "aLabel", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIPaymentCurrencyAmount amount; */
                    Method {
                        name: "GetAmount",
                        params: &[Param { name: "aAmount", ty: "*mut *const nsIPaymentCurrencyAmount" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean pending; */
                    Method {
                        name: "GetPending",
                        params: &[Param { name: "aPending", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIPaymentDetailsModifier",
            base: Some("nsISupports"),
            methods: Err("specialtype jsval unsupported"),
        },

        Interface {
            name: "nsIPaymentShippingOption",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute AString id; */
                    Method {
                        name: "GetId",
                        params: &[Param { name: "aId", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString label; */
                    Method {
                        name: "GetLabel",
                        params: &[Param { name: "aLabel", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIPaymentCurrencyAmount amount; */
                    Method {
                        name: "GetAmount",
                        params: &[Param { name: "aAmount", ty: "*mut *const nsIPaymentCurrencyAmount" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute boolean selected; */
                    Method {
                        name: "GetSelected",
                        params: &[Param { name: "aSelected", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetSelected",
                        params: &[Param { name: "aSelected", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIPaymentDetails",
            base: Some("nsISupports"),
            methods: Err("specialtype jsval unsupported"),
        },

        Interface {
            name: "nsIPaymentOptions",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute boolean requestPayerName; */
                    Method {
                        name: "GetRequestPayerName",
                        params: &[Param { name: "aRequestPayerName", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean requestPayerEmail; */
                    Method {
                        name: "GetRequestPayerEmail",
                        params: &[Param { name: "aRequestPayerEmail", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean requestPayerPhone; */
                    Method {
                        name: "GetRequestPayerPhone",
                        params: &[Param { name: "aRequestPayerPhone", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean requestShipping; */
                    Method {
                        name: "GetRequestShipping",
                        params: &[Param { name: "aRequestShipping", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean requestBillingAddress; */
                    Method {
                        name: "GetRequestBillingAddress",
                        params: &[Param { name: "aRequestBillingAddress", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString shippingType; */
                    Method {
                        name: "GetShippingType",
                        params: &[Param { name: "aShippingType", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIPaymentRequest",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute uint64_t topOuterWindowId; */
                    Method {
                        name: "GetTopOuterWindowId",
                        params: &[Param { name: "aTopOuterWindowId", ty: "*mut uint64_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIPrincipal topLevelPrincipal; */
                    Method {
                        name: "GetTopLevelPrincipal",
                        params: &[Param { name: "aTopLevelPrincipal", ty: "*mut *const nsIPrincipal" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString requestId; */
                    Method {
                        name: "GetRequestId",
                        params: &[Param { name: "aRequestId", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString completeStatus; */
                    Method {
                        name: "GetCompleteStatus",
                        params: &[Param { name: "aCompleteStatus", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIArray paymentMethods; */
                    Method {
                        name: "GetPaymentMethods",
                        params: &[Param { name: "aPaymentMethods", ty: "*mut*const nsIArray" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIPaymentDetails paymentDetails; */
                    Method {
                        name: "GetPaymentDetails",
                        params: &[Param { name: "aPaymentDetails", ty: "*mut *const nsIPaymentDetails" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIPaymentOptions paymentOptions; */
                    Method {
                        name: "GetPaymentOptions",
                        params: &[Param { name: "aPaymentOptions", ty: "*mut *const nsIPaymentOptions" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString shippingOption; */
                    Method {
                        name: "GetShippingOption",
                        params: &[Param { name: "aShippingOption", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

