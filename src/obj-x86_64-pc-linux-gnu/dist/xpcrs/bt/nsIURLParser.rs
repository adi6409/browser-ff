//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIURLParser.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIURLParser",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void parseURL (in string spec, in long specLen, out unsigned long schemePos, out long schemeLen, out unsigned long authorityPos, out long authorityLen, out unsigned long pathPos, out long pathLen); */
                    Method {
                        name: "ParseURL",
                        params: &[Param { name: "spec", ty: "*const libc::c_char" }, Param { name: "specLen", ty: "i32" }, Param { name: "schemePos", ty: "*mut u32" }, Param { name: "schemeLen", ty: "*mut i32" }, Param { name: "authorityPos", ty: "*mut u32" }, Param { name: "authorityLen", ty: "*mut i32" }, Param { name: "pathPos", ty: "*mut u32" }, Param { name: "pathLen", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void parseAuthority (in string authority, in long authorityLen, out unsigned long usernamePos, out long usernameLen, out unsigned long passwordPos, out long passwordLen, out unsigned long hostnamePos, out long hostnameLen, out long port); */
                    Method {
                        name: "ParseAuthority",
                        params: &[Param { name: "authority", ty: "*const libc::c_char" }, Param { name: "authorityLen", ty: "i32" }, Param { name: "usernamePos", ty: "*mut u32" }, Param { name: "usernameLen", ty: "*mut i32" }, Param { name: "passwordPos", ty: "*mut u32" }, Param { name: "passwordLen", ty: "*mut i32" }, Param { name: "hostnamePos", ty: "*mut u32" }, Param { name: "hostnameLen", ty: "*mut i32" }, Param { name: "port", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void parseUserInfo (in string userinfo, in long userinfoLen, out unsigned long usernamePos, out long usernameLen, out unsigned long passwordPos, out long passwordLen); */
                    Method {
                        name: "ParseUserInfo",
                        params: &[Param { name: "userinfo", ty: "*const libc::c_char" }, Param { name: "userinfoLen", ty: "i32" }, Param { name: "usernamePos", ty: "*mut u32" }, Param { name: "usernameLen", ty: "*mut i32" }, Param { name: "passwordPos", ty: "*mut u32" }, Param { name: "passwordLen", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void parseServerInfo (in string serverinfo, in long serverinfoLen, out unsigned long hostnamePos, out long hostnameLen, out long port); */
                    Method {
                        name: "ParseServerInfo",
                        params: &[Param { name: "serverinfo", ty: "*const libc::c_char" }, Param { name: "serverinfoLen", ty: "i32" }, Param { name: "hostnamePos", ty: "*mut u32" }, Param { name: "hostnameLen", ty: "*mut i32" }, Param { name: "port", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void parsePath (in string path, in long pathLen, out unsigned long filepathPos, out long filepathLen, out unsigned long queryPos, out long queryLen, out unsigned long refPos, out long refLen); */
                    Method {
                        name: "ParsePath",
                        params: &[Param { name: "path", ty: "*const libc::c_char" }, Param { name: "pathLen", ty: "i32" }, Param { name: "filepathPos", ty: "*mut u32" }, Param { name: "filepathLen", ty: "*mut i32" }, Param { name: "queryPos", ty: "*mut u32" }, Param { name: "queryLen", ty: "*mut i32" }, Param { name: "refPos", ty: "*mut u32" }, Param { name: "refLen", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void parseFilePath (in string filepath, in long filepathLen, out unsigned long directoryPos, out long directoryLen, out unsigned long basenamePos, out long basenameLen, out unsigned long extensionPos, out long extensionLen); */
                    Method {
                        name: "ParseFilePath",
                        params: &[Param { name: "filepath", ty: "*const libc::c_char" }, Param { name: "filepathLen", ty: "i32" }, Param { name: "directoryPos", ty: "*mut u32" }, Param { name: "directoryLen", ty: "*mut i32" }, Param { name: "basenamePos", ty: "*mut u32" }, Param { name: "basenameLen", ty: "*mut i32" }, Param { name: "extensionPos", ty: "*mut u32" }, Param { name: "extensionLen", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void parseFileName (in string filename, in long filenameLen, out unsigned long basenamePos, out long basenameLen, out unsigned long extensionPos, out long extensionLen); */
                    Method {
                        name: "ParseFileName",
                        params: &[Param { name: "filename", ty: "*const libc::c_char" }, Param { name: "filenameLen", ty: "i32" }, Param { name: "basenamePos", ty: "*mut u32" }, Param { name: "basenameLen", ty: "*mut i32" }, Param { name: "extensionPos", ty: "*mut u32" }, Param { name: "extensionLen", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

