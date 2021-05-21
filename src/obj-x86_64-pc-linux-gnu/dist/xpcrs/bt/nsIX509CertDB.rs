//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsIX509CertDB.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIOpenSignedAppFileCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void openSignedAppFileFinished (in nsresult rv, in nsIZipReader aZipReader, in nsIX509Cert aSignerCert); */
                    Method {
                        name: "OpenSignedAppFileFinished",
                        params: &[Param { name: "rv", ty: "::nserror::nsresult" }, Param { name: "aZipReader", ty: "*const nsIZipReader" }, Param { name: "aSignerCert", ty: "*const nsIX509Cert" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIAsyncBoolCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onResult (in bool result); */
                    Method {
                        name: "OnResult",
                        params: &[Param { name: "result", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsICertVerificationCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void verifyCertFinished (in int32_t aPRErrorCode, in Array<nsIX509Cert> aVerifiedChain, in bool aHasEVPolicy); */
                    Method {
                        name: "VerifyCertFinished",
                        params: &[Param { name: "aPRErrorCode", ty: "int32_t" }, Param { name: "aVerifiedChain", ty: "*const thin_vec::ThinVec<RefPtr<nsIX509Cert>>" }, Param { name: "aHasEVPolicy", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIX509CertDB",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* [must_use] nsIX509Cert findCertByDBKey (in ACString aDBkey); */
                    Method {
                        name: "FindCertByDBKey",
                        params: &[Param { name: "aDBkey", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut*const nsIX509Cert" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void importCertificates ([array, size_is (length)] in octet data, in unsigned long length, in unsigned long type, in nsIInterfaceRequestor ctx); */
                    Method {
                        name: "ImportCertificates",
                        params: &[Param { name: "data", ty: "*mut u8" }, Param { name: "length", ty: "u32" }, Param { name: "type_", ty: "u32" }, Param { name: "ctx", ty: "*const nsIInterfaceRequestor" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void importEmailCertificate ([array, size_is (length)] in octet data, in unsigned long length, in nsIInterfaceRequestor ctx); */
                    Method {
                        name: "ImportEmailCertificate",
                        params: &[Param { name: "data", ty: "*mut u8" }, Param { name: "length", ty: "u32" }, Param { name: "ctx", ty: "*const nsIInterfaceRequestor" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void importUserCertificate ([array, size_is (length)] in octet data, in unsigned long length, in nsIInterfaceRequestor ctx); */
                    Method {
                        name: "ImportUserCertificate",
                        params: &[Param { name: "data", ty: "*mut u8" }, Param { name: "length", ty: "u32" }, Param { name: "ctx", ty: "*const nsIInterfaceRequestor" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void deleteCertificate (in nsIX509Cert aCert); */
                    Method {
                        name: "DeleteCertificate",
                        params: &[Param { name: "aCert", ty: "*const nsIX509Cert" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void setCertTrust (in nsIX509Cert cert, in unsigned long type, in unsigned long trust); */
                    Method {
                        name: "SetCertTrust",
                        params: &[Param { name: "cert", ty: "*const nsIX509Cert" }, Param { name: "type_", ty: "u32" }, Param { name: "trust", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void setCertTrustFromString (in nsIX509Cert cert, in ACString trustString); */
                    Method {
                        name: "SetCertTrustFromString",
                        params: &[Param { name: "cert", ty: "*const nsIX509Cert" }, Param { name: "trustString", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] boolean isCertTrusted (in nsIX509Cert cert, in unsigned long certType, in unsigned long trustType); */
                    Method {
                        name: "IsCertTrusted",
                        params: &[Param { name: "cert", ty: "*const nsIX509Cert" }, Param { name: "certType", ty: "u32" }, Param { name: "trustType", ty: "u32" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void importCertsFromFile (in nsIFile aFile, in unsigned long aType); */
                    Method {
                        name: "ImportCertsFromFile",
                        params: &[Param { name: "aFile", ty: "*const nsIFile" }, Param { name: "aType", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] uint32_t importPKCS12File (in nsIFile aFile, in AString aPassword); */
                    Method {
                        name: "ImportPKCS12File",
                        params: &[Param { name: "aFile", ty: "*const nsIFile" }, Param { name: "aPassword", ty: "*const ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] uint32_t exportPKCS12File (in nsIFile aFile, in Array<nsIX509Cert> aCerts, in AString aPassword); */
                    Method {
                        name: "ExportPKCS12File",
                        params: &[Param { name: "aFile", ty: "*const nsIFile" }, Param { name: "aCerts", ty: "*const thin_vec::ThinVec<RefPtr<nsIX509Cert>>" }, Param { name: "aPassword", ty: "*const ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] nsIX509Cert constructX509FromBase64 (in ACString base64); */
                    Method {
                        name: "ConstructX509FromBase64",
                        params: &[Param { name: "base64", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut*const nsIX509Cert" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] nsIX509Cert constructX509 (in Array<uint8_t> certDER); */
                    Method {
                        name: "ConstructX509",
                        params: &[Param { name: "certDER", ty: "*const thin_vec::ThinVec<uint8_t>" }, Param { name: "_retval", ty: "*mut*const nsIX509Cert" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void openSignedAppFileAsync (in AppTrustedRoot trustedRoot, in nsIFile aJarFile, in nsIOpenSignedAppFileCallback callback); */
                    Method {
                        name: "OpenSignedAppFileAsync",
                        params: &[Param { name: "trustedRoot", ty: "AppTrustedRoot" }, Param { name: "aJarFile", ty: "*const nsIFile" }, Param { name: "callback", ty: "*const nsIOpenSignedAppFileCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] nsIX509Cert addCert (in ACString certDER, in ACString trust); */
                    Method {
                        name: "AddCert",
                        params: &[Param { name: "certDER", ty: "*const ::nsstring::nsACString" }, Param { name: "trust", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut*const nsIX509Cert" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void asyncVerifyCertAtTime (in nsIX509Cert aCert, in int64_t aUsage, in uint32_t aFlags, in ACString aHostname, in uint64_t aTime, in nsICertVerificationCallback aCallback); */
                    Method {
                        name: "AsyncVerifyCertAtTime",
                        params: &[Param { name: "aCert", ty: "*const nsIX509Cert" }, Param { name: "aUsage", ty: "int64_t" }, Param { name: "aFlags", ty: "uint32_t" }, Param { name: "aHostname", ty: "*const ::nsstring::nsACString" }, Param { name: "aTime", ty: "uint64_t" }, Param { name: "aCallback", ty: "*const nsICertVerificationCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void clearOCSPCache (); */
                    Method {
                        name: "ClearOCSPCache",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] nsIX509Cert addCertFromBase64 (in ACString base64, in ACString trust); */
                    Method {
                        name: "AddCertFromBase64",
                        params: &[Param { name: "base64", ty: "*const ::nsstring::nsACString" }, Param { name: "trust", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut*const nsIX509Cert" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] Array<nsIX509Cert> getCerts (); */
                    Method {
                        name: "GetCerts",
                        params: &[Param { name: "_retval", ty: "*mut thin_vec::ThinVec<RefPtr<nsIX509Cert>>" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] ACString asPKCS7Blob (in Array<nsIX509Cert> certList); */
                    Method {
                        name: "AsPKCS7Blob",
                        params: &[Param { name: "certList", ty: "*const thin_vec::ThinVec<RefPtr<nsIX509Cert>>" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void asyncHasThirdPartyRoots (in nsIAsyncBoolCallback callback); */
                    Method {
                        name: "AsyncHasThirdPartyRoots",
                        params: &[Param { name: "callback", ty: "*const nsIAsyncBoolCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

