//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/modules/nsIE10SUtils.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIE10SUtils",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* AUTF8String getRemoteTypeForPrincipal (in nsIPrincipal aPrincipal, in nsIURI aChannelOriginalURI, in boolean aMultiProcess, in boolean aRemoteSubframes, in AUTF8String aPreferredRemoteType, in nsIPrincipal aCurrentPrincipal, in boolean aIsSubframe); */
                    Method {
                        name: "GetRemoteTypeForPrincipal",
                        params: &[Param { name: "aPrincipal", ty: "*const nsIPrincipal" }, Param { name: "aChannelOriginalURI", ty: "*const nsIURI" }, Param { name: "aMultiProcess", ty: "bool" }, Param { name: "aRemoteSubframes", ty: "bool" }, Param { name: "aPreferredRemoteType", ty: "*const ::nsstring::nsACString" }, Param { name: "aCurrentPrincipal", ty: "*const nsIPrincipal" }, Param { name: "aIsSubframe", ty: "bool" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AUTF8String getRemoteTypeForWorkerPrincipal (in nsIPrincipal aPrincipal, in nsIE10SUtils_RemoteWorkerType aWorkerType, in boolean aIsMultiProcess, in boolean aIsFission, in AUTF8String aPreferredRemoteType); */
                    Method {
                        name: "GetRemoteTypeForWorkerPrincipal",
                        params: &[Param { name: "aPrincipal", ty: "*const nsIPrincipal" }, Param { name: "aWorkerType", ty: " u8" }, Param { name: "aIsMultiProcess", ty: "bool" }, Param { name: "aIsFission", ty: "bool" }, Param { name: "aPreferredRemoteType", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

