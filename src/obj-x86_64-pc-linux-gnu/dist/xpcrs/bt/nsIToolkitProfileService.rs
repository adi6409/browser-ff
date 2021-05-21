//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/profile/nsIToolkitProfileService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIToolkitProfileService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* [infallible] readonly attribute boolean isListOutdated; */
                    Method {
                        name: "GetIsListOutdated",
                        params: &[Param { name: "aIsListOutdated", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute boolean startWithLastProfile; */
                    Method {
                        name: "GetStartWithLastProfile",
                        params: &[Param { name: "aStartWithLastProfile", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetStartWithLastProfile",
                        params: &[Param { name: "aStartWithLastProfile", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsISimpleEnumerator profiles; */
                    Method {
                        name: "GetProfiles",
                        params: &[Param { name: "aProfiles", ty: "*mut*const nsISimpleEnumerator" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIToolkitProfile currentProfile; */
                    Method {
                        name: "GetCurrentProfile",
                        params: &[Param { name: "aCurrentProfile", ty: "*mut*const nsIToolkitProfile" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute nsIToolkitProfile defaultProfile; */
                    Method {
                        name: "GetDefaultProfile",
                        params: &[Param { name: "aDefaultProfile", ty: "*mut*const nsIToolkitProfile" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetDefaultProfile",
                        params: &[Param { name: "aDefaultProfile", ty: "*const nsIToolkitProfile" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean createdAlternateProfile; */
                    Method {
                        name: "GetCreatedAlternateProfile",
                        params: &[Param { name: "aCreatedAlternateProfile", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* bool selectStartupProfile (in Array<ACString> aArgv, in boolean aIsResetting, in AUTF8String aUpdateChannel, in AUTF8String aLegacyInstallHash, out nsIFile aRootDir, out nsIFile aLocalDir, out nsIToolkitProfile aProfile); */
                    Method {
                        name: "SelectStartupProfile",
                        params: &[Param { name: "aArgv", ty: "*const thin_vec::ThinVec<::nsstring::nsCString>" }, Param { name: "aIsResetting", ty: "bool" }, Param { name: "aUpdateChannel", ty: "*const ::nsstring::nsACString" }, Param { name: "aLegacyInstallHash", ty: "*const ::nsstring::nsACString" }, Param { name: "aRootDir", ty: "*mut*const nsIFile" }, Param { name: "aLocalDir", ty: "*mut*const nsIFile" }, Param { name: "aProfile", ty: "*mut*const nsIToolkitProfile" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIToolkitProfile getProfileByName (in AUTF8String aName); */
                    Method {
                        name: "GetProfileByName",
                        params: &[Param { name: "aName", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut*const nsIToolkitProfile" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIToolkitProfile createProfile (in nsIFile aRootDir, in AUTF8String aName); */
                    Method {
                        name: "CreateProfile",
                        params: &[Param { name: "aRootDir", ty: "*const nsIFile" }, Param { name: "aName", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut*const nsIToolkitProfile" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIToolkitProfile createUniqueProfile (in nsIFile aRootDir, in AUTF8String aNamePrefix); */
                    Method {
                        name: "CreateUniqueProfile",
                        params: &[Param { name: "aRootDir", ty: "*const nsIFile" }, Param { name: "aNamePrefix", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut*const nsIToolkitProfile" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long profileCount; */
                    Method {
                        name: "GetProfileCount",
                        params: &[Param { name: "aProfileCount", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void flush (); */
                    Method {
                        name: "Flush",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

