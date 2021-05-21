//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/quota/nsIQuotaCallbacks.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIQuotaUsageCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onUsageResult (in nsIQuotaUsageRequest aRequest); */
                    Method {
                        name: "OnUsageResult",
                        params: &[Param { name: "aRequest", ty: "*const nsIQuotaUsageRequest" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIQuotaCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onComplete (in nsIQuotaRequest aRequest); */
                    Method {
                        name: "OnComplete",
                        params: &[Param { name: "aRequest", ty: "*const nsIQuotaRequest" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

