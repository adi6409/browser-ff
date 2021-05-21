//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/profile/nsIToolkitProfile.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIProfileLock",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute nsIFile directory; */
                    Method {
                        name: "GetDirectory",
                        params: &[Param { name: "aDirectory", ty: "*mut*const nsIFile" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIFile localDirectory; */
                    Method {
                        name: "GetLocalDirectory",
                        params: &[Param { name: "aLocalDirectory", ty: "*mut*const nsIFile" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute PRTime replacedLockTime; */
                    Method {
                        name: "GetReplacedLockTime",
                        params: &[Param { name: "aReplacedLockTime", ty: "*mut PRTime" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void unlock (); */
                    Method {
                        name: "Unlock",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIToolkitProfile",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute nsIFile rootDir; */
                    Method {
                        name: "GetRootDir",
                        params: &[Param { name: "aRootDir", ty: "*mut*const nsIFile" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIFile localDir; */
                    Method {
                        name: "GetLocalDir",
                        params: &[Param { name: "aLocalDir", ty: "*mut*const nsIFile" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute AUTF8String name; */
                    Method {
                        name: "GetName",
                        params: &[Param { name: "aName", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetName",
                        params: &[Param { name: "aName", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void remove (in boolean removeFiles); */
                    Method {
                        name: "Remove",
                        params: &[Param { name: "removeFiles", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void removeInBackground (in boolean removeFiles); */
                    Method {
                        name: "RemoveInBackground",
                        params: &[Param { name: "removeFiles", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIProfileLock lock (out nsIProfileUnlocker aUnlocker); */
                    Method {
                        name: "Lock",
                        params: &[Param { name: "aUnlocker", ty: "*mut*const nsIProfileUnlocker" }, Param { name: "_retval", ty: "*mut *const nsIProfileLock" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

