//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsINSSErrorsService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsINSSErrorsService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* [must_use] boolean isNSSErrorCode (in int32_t aNSPRCode); */
                    Method {
                        name: "IsNSSErrorCode",
                        params: &[Param { name: "aNSPRCode", ty: "int32_t" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] nsresult getXPCOMFromNSSError (in int32_t aNSPRCode); */
                    Method {
                        name: "GetXPCOMFromNSSError",
                        params: &[Param { name: "aNSPRCode", ty: "int32_t" }, Param { name: "_retval", ty: "*mut ::nserror::nsresult" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString getErrorMessage (in nsresult aXPCOMErrorCode); */
                    Method {
                        name: "GetErrorMessage",
                        params: &[Param { name: "aXPCOMErrorCode", ty: "::nserror::nsresult" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] uint32_t getErrorClass (in nsresult aXPCOMErrorCode); */
                    Method {
                        name: "GetErrorClass",
                        params: &[Param { name: "aXPCOMErrorCode", ty: "::nserror::nsresult" }, Param { name: "_retval", ty: "*mut uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

