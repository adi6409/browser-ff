//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/payments/nsIPaymentActionResponse.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPaymentResponseData",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute uint32_t type; */
                    Method {
                        name: "GetType",
                        params: &[Param { name: "aType", ty: "*mut uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void init (in uint32_t aType); */
                    Method {
                        name: "Init",
                        params: &[Param { name: "aType", ty: "uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIGeneralResponseData",
            base: Some("nsIPaymentResponseData"),
            methods: Err("specialtype jsval unsupported"),
        },

        Interface {
            name: "nsIBasicCardResponseData",
            base: Some("nsIPaymentResponseData"),
            methods: Ok(&[
                    /* readonly attribute AString cardholderName; */
                    Method {
                        name: "GetCardholderName",
                        params: &[Param { name: "aCardholderName", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString cardNumber; */
                    Method {
                        name: "GetCardNumber",
                        params: &[Param { name: "aCardNumber", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString expiryMonth; */
                    Method {
                        name: "GetExpiryMonth",
                        params: &[Param { name: "aExpiryMonth", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString expiryYear; */
                    Method {
                        name: "GetExpiryYear",
                        params: &[Param { name: "aExpiryYear", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString cardSecurityCode; */
                    Method {
                        name: "GetCardSecurityCode",
                        params: &[Param { name: "aCardSecurityCode", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIPaymentAddress billingAddress; */
                    Method {
                        name: "GetBillingAddress",
                        params: &[Param { name: "aBillingAddress", ty: "*mut *const nsIPaymentAddress" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void initData (in AString aCardholderName, in AString aCardNumber, in AString aExpiryMonth, in AString aExpiryYear, in AString aCardSecurityCode, in nsIPaymentAddress billingAddress); */
                    Method {
                        name: "InitData",
                        params: &[Param { name: "aCardholderName", ty: "*const ::nsstring::nsAString" }, Param { name: "aCardNumber", ty: "*const ::nsstring::nsAString" }, Param { name: "aExpiryMonth", ty: "*const ::nsstring::nsAString" }, Param { name: "aExpiryYear", ty: "*const ::nsstring::nsAString" }, Param { name: "aCardSecurityCode", ty: "*const ::nsstring::nsAString" }, Param { name: "billingAddress", ty: "*const nsIPaymentAddress" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIPaymentActionResponse",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute AString requestId; */
                    Method {
                        name: "GetRequestId",
                        params: &[Param { name: "aRequestId", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute uint32_t type; */
                    Method {
                        name: "GetType",
                        params: &[Param { name: "aType", ty: "*mut uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIPaymentCanMakeActionResponse",
            base: Some("nsIPaymentActionResponse"),
            methods: Ok(&[
                    /* readonly attribute bool result; */
                    Method {
                        name: "GetResult",
                        params: &[Param { name: "aResult", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void init (in AString aRequestId, in bool aResult); */
                    Method {
                        name: "Init",
                        params: &[Param { name: "aRequestId", ty: "*const ::nsstring::nsAString" }, Param { name: "aResult", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIPaymentShowActionResponse",
            base: Some("nsIPaymentActionResponse"),
            methods: Ok(&[
                    /* readonly attribute uint32_t acceptStatus; */
                    Method {
                        name: "GetAcceptStatus",
                        params: &[Param { name: "aAcceptStatus", ty: "*mut uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString methodName; */
                    Method {
                        name: "GetMethodName",
                        params: &[Param { name: "aMethodName", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIPaymentResponseData data; */
                    Method {
                        name: "GetData",
                        params: &[Param { name: "aData", ty: "*mut *const nsIPaymentResponseData" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString payerName; */
                    Method {
                        name: "GetPayerName",
                        params: &[Param { name: "aPayerName", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString payerEmail; */
                    Method {
                        name: "GetPayerEmail",
                        params: &[Param { name: "aPayerEmail", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString payerPhone; */
                    Method {
                        name: "GetPayerPhone",
                        params: &[Param { name: "aPayerPhone", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void init (in AString aRequestId, in uint32_t aAcceptStatus, in AString aMethodName, in nsIPaymentResponseData aData, in AString aPayerName, in AString aPayerEmail, in AString aPayerPhone); */
                    Method {
                        name: "Init",
                        params: &[Param { name: "aRequestId", ty: "*const ::nsstring::nsAString" }, Param { name: "aAcceptStatus", ty: "uint32_t" }, Param { name: "aMethodName", ty: "*const ::nsstring::nsAString" }, Param { name: "aData", ty: "*const nsIPaymentResponseData" }, Param { name: "aPayerName", ty: "*const ::nsstring::nsAString" }, Param { name: "aPayerEmail", ty: "*const ::nsstring::nsAString" }, Param { name: "aPayerPhone", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIPaymentAbortActionResponse",
            base: Some("nsIPaymentActionResponse"),
            methods: Ok(&[
                    /* readonly attribute uint32_t abortStatus; */
                    Method {
                        name: "GetAbortStatus",
                        params: &[Param { name: "aAbortStatus", ty: "*mut uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void init (in AString aRequestId, in uint32_t aAbortStatus); */
                    Method {
                        name: "Init",
                        params: &[Param { name: "aRequestId", ty: "*const ::nsstring::nsAString" }, Param { name: "aAbortStatus", ty: "uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* bool isSucceeded (); */
                    Method {
                        name: "IsSucceeded",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIPaymentCompleteActionResponse",
            base: Some("nsIPaymentActionResponse"),
            methods: Ok(&[
                    /* readonly attribute uint32_t completeStatus; */
                    Method {
                        name: "GetCompleteStatus",
                        params: &[Param { name: "aCompleteStatus", ty: "*mut uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void init (in AString aRequestId, in uint32_t aCompleteStatus); */
                    Method {
                        name: "Init",
                        params: &[Param { name: "aRequestId", ty: "*const ::nsstring::nsAString" }, Param { name: "aCompleteStatus", ty: "uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* bool isCompleted (); */
                    Method {
                        name: "IsCompleted",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIMethodChangeDetails",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute uint32_t type; */
                    Method {
                        name: "GetType",
                        params: &[Param { name: "aType", ty: "*mut uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void init (in uint32_t aType); */
                    Method {
                        name: "Init",
                        params: &[Param { name: "aType", ty: "uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIGeneralChangeDetails",
            base: Some("nsIMethodChangeDetails"),
            methods: Err("specialtype jsval unsupported"),
        },

        Interface {
            name: "nsIBasicCardChangeDetails",
            base: Some("nsIMethodChangeDetails"),
            methods: Ok(&[
                    /* readonly attribute nsIPaymentAddress billingAddress; */
                    Method {
                        name: "GetBillingAddress",
                        params: &[Param { name: "aBillingAddress", ty: "*mut *const nsIPaymentAddress" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void initData (in nsIPaymentAddress billingAddress); */
                    Method {
                        name: "InitData",
                        params: &[Param { name: "billingAddress", ty: "*const nsIPaymentAddress" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

