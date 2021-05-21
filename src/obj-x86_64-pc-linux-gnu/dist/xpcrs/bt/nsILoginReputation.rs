//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/reputationservice/nsILoginReputation.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsILoginReputationVerdictType",
            base: Some("nsISupports"),
            methods: Ok(&[
                    ]),
        },

        Interface {
            name: "nsILoginReputationQuery",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute nsIURI formURI; */
                    Method {
                        name: "GetFormURI",
                        params: &[Param { name: "aFormURI", ty: "*mut*const nsIURI" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsILoginReputationQueryCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onComplete (in nsresult aStatus, in unsigned long aVerdict); */
                    Method {
                        name: "OnComplete",
                        params: &[Param { name: "aStatus", ty: "::nserror::nsresult" }, Param { name: "aVerdict", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsILoginReputationService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void init (); */
                    Method {
                        name: "Init",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void queryReputationAsync (in HTMLInputElement aInput, in nsILoginReputationQueryCallback aCallback); */
                    Method {
                        name: "QueryReputationAsync",
                        params: &[Param { name: "aInput", ty: "*const libc::c_void" }, Param { name: "aCallback", ty: "*const nsILoginReputationQueryCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void queryReputation (in nsILoginReputationQuery aQuery, in nsILoginReputationQueryCallback aCallback); */
                    Method {
                        name: "QueryReputation",
                        params: &[Param { name: "aQuery", ty: "*const nsILoginReputationQuery" }, Param { name: "aCallback", ty: "*const nsILoginReputationQueryCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

