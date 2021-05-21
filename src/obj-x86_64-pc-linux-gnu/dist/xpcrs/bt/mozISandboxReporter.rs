//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/sandbox/linux/interfaces/mozISandboxReporter.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "mozISandboxReport",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute uint64_t msecAgo; */
                    Method {
                        name: "GetMsecAgo",
                        params: &[Param { name: "aMsecAgo", ty: "*mut uint64_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute int32_t pid; */
                    Method {
                        name: "GetPid",
                        params: &[Param { name: "aPid", ty: "*mut int32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute int32_t tid; */
                    Method {
                        name: "GetTid",
                        params: &[Param { name: "aTid", ty: "*mut int32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute ACString procType; */
                    Method {
                        name: "GetProcType",
                        params: &[Param { name: "aProcType", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute uint32_t syscall; */
                    Method {
                        name: "GetSyscall",
                        params: &[Param { name: "aSyscall", ty: "*mut uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute uint32_t numArgs; */
                    Method {
                        name: "GetNumArgs",
                        params: &[Param { name: "aNumArgs", ty: "*mut uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* ACString getArg (in uint32_t aIndex); */
                    Method {
                        name: "GetArg",
                        params: &[Param { name: "aIndex", ty: "uint32_t" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "mozISandboxReportArray",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute uint64_t begin; */
                    Method {
                        name: "GetBegin",
                        params: &[Param { name: "aBegin", ty: "*mut uint64_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute uint64_t end; */
                    Method {
                        name: "GetEnd",
                        params: &[Param { name: "aEnd", ty: "*mut uint64_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* mozISandboxReport getElement (in uint64_t aIndex); */
                    Method {
                        name: "GetElement",
                        params: &[Param { name: "aIndex", ty: "uint64_t" }, Param { name: "_retval", ty: "*mut *const mozISandboxReport" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "mozISandboxReporter",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* mozISandboxReportArray snapshot (); */
                    Method {
                        name: "Snapshot",
                        params: &[Param { name: "_retval", ty: "*mut *const mozISandboxReportArray" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

