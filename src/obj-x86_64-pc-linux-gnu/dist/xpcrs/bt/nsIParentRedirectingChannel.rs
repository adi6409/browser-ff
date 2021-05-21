//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIParentRedirectingChannel.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAsyncVerifyRedirectReadyCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void readyToVerify (in nsresult result); */
                    Method {
                        name: "ReadyToVerify",
                        params: &[Param { name: "result", ty: "::nserror::nsresult" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIParentRedirectingChannel",
            base: Some("nsIParentChannel"),
            methods: Ok(&[
                    /* void startRedirect (in nsIChannel newChannel, in uint32_t redirectFlags, in nsIAsyncVerifyRedirectCallback callback); */
                    Method {
                        name: "StartRedirect",
                        params: &[Param { name: "newChannel", ty: "*const nsIChannel" }, Param { name: "redirectFlags", ty: "uint32_t" }, Param { name: "callback", ty: "*const nsIAsyncVerifyRedirectCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void continueVerification (in nsIAsyncVerifyRedirectReadyCallback callback); */
                    Method {
                        name: "ContinueVerification",
                        params: &[Param { name: "callback", ty: "*const nsIAsyncVerifyRedirectReadyCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void completeRedirect (in boolean succeeded); */
                    Method {
                        name: "CompleteRedirect",
                        params: &[Param { name: "succeeded", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

