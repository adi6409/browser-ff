//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIBackgroundFileSaver.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIBackgroundFileSaver",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* attribute nsIBackgroundFileSaverObserver observer; */
                    Method {
                        name: "GetObserver",
                        params: &[Param { name: "aObserver", ty: "*mut*const nsIBackgroundFileSaverObserver" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetObserver",
                        params: &[Param { name: "aObserver", ty: "*const nsIBackgroundFileSaverObserver" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute Array<Array<Array<uint8_t>>> signatureInfo; */
                    Method {
                        name: "GetSignatureInfo",
                        params: &[Param { name: "aSignatureInfo", ty: "*mut thin_vec::ThinVec<thin_vec::ThinVec<thin_vec::ThinVec<uint8_t>>>" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute ACString sha256Hash; */
                    Method {
                        name: "GetSha256Hash",
                        params: &[Param { name: "aSha256Hash", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void enableSignatureInfo (); */
                    Method {
                        name: "EnableSignatureInfo",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void enableSha256 (); */
                    Method {
                        name: "EnableSha256",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void enableAppend (); */
                    Method {
                        name: "EnableAppend",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void setTarget (in nsIFile aTarget, in bool aKeepPartial); */
                    Method {
                        name: "SetTarget",
                        params: &[Param { name: "aTarget", ty: "*const nsIFile" }, Param { name: "aKeepPartial", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void finish (in nsresult aStatus); */
                    Method {
                        name: "Finish",
                        params: &[Param { name: "aStatus", ty: "::nserror::nsresult" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIBackgroundFileSaverObserver",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onTargetChange (in nsIBackgroundFileSaver aSaver, in nsIFile aTarget); */
                    Method {
                        name: "OnTargetChange",
                        params: &[Param { name: "aSaver", ty: "*const nsIBackgroundFileSaver" }, Param { name: "aTarget", ty: "*const nsIFile" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void onSaveComplete (in nsIBackgroundFileSaver aSaver, in nsresult aStatus); */
                    Method {
                        name: "OnSaveComplete",
                        params: &[Param { name: "aSaver", ty: "*const nsIBackgroundFileSaver" }, Param { name: "aStatus", ty: "::nserror::nsresult" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

