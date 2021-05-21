/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/sandbox/linux/interfaces/mozISandboxReporter.idl
 */

#ifndef __gen_mozISandboxReporter_h__
#define __gen_mozISandboxReporter_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    mozISandboxReport */
#define MOZISANDBOXREPORT_IID_STR "ed1e84d3-3346-42e1-b28c-e76a77f549f0"

#define MOZISANDBOXREPORT_IID \
  {0xed1e84d3, 0x3346, 0x42e1, \
    { 0xb2, 0x8c, 0xe7, 0x6a, 0x77, 0xf5, 0x49, 0xf0 }}

class NS_NO_VTABLE mozISandboxReport : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(MOZISANDBOXREPORT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = mozISandboxReport;

  /* readonly attribute uint64_t msecAgo; */
  NS_IMETHOD GetMsecAgo(uint64_t *aMsecAgo) = 0;

  /* readonly attribute int32_t pid; */
  NS_IMETHOD GetPid(int32_t *aPid) = 0;

  /* readonly attribute int32_t tid; */
  NS_IMETHOD GetTid(int32_t *aTid) = 0;

  /* readonly attribute ACString procType; */
  NS_IMETHOD GetProcType(nsACString& aProcType) = 0;

  /* readonly attribute uint32_t syscall; */
  NS_IMETHOD GetSyscall(uint32_t *aSyscall) = 0;

  /* readonly attribute uint32_t numArgs; */
  NS_IMETHOD GetNumArgs(uint32_t *aNumArgs) = 0;

  /* ACString getArg (in uint32_t aIndex); */
  NS_IMETHOD GetArg(uint32_t aIndex, nsACString& _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(mozISandboxReport, MOZISANDBOXREPORT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_MOZISANDBOXREPORT \
  NS_IMETHOD GetMsecAgo(uint64_t *aMsecAgo) override; \
  NS_IMETHOD GetPid(int32_t *aPid) override; \
  NS_IMETHOD GetTid(int32_t *aTid) override; \
  NS_IMETHOD GetProcType(nsACString& aProcType) override; \
  NS_IMETHOD GetSyscall(uint32_t *aSyscall) override; \
  NS_IMETHOD GetNumArgs(uint32_t *aNumArgs) override; \
  NS_IMETHOD GetArg(uint32_t aIndex, nsACString& _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_MOZISANDBOXREPORT \
  nsresult GetMsecAgo(uint64_t *aMsecAgo); \
  nsresult GetPid(int32_t *aPid); \
  nsresult GetTid(int32_t *aTid); \
  nsresult GetProcType(nsACString& aProcType); \
  nsresult GetSyscall(uint32_t *aSyscall); \
  nsresult GetNumArgs(uint32_t *aNumArgs); \
  nsresult GetArg(uint32_t aIndex, nsACString& _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_MOZISANDBOXREPORT(_to) \
  NS_IMETHOD GetMsecAgo(uint64_t *aMsecAgo) override { return _to GetMsecAgo(aMsecAgo); } \
  NS_IMETHOD GetPid(int32_t *aPid) override { return _to GetPid(aPid); } \
  NS_IMETHOD GetTid(int32_t *aTid) override { return _to GetTid(aTid); } \
  NS_IMETHOD GetProcType(nsACString& aProcType) override { return _to GetProcType(aProcType); } \
  NS_IMETHOD GetSyscall(uint32_t *aSyscall) override { return _to GetSyscall(aSyscall); } \
  NS_IMETHOD GetNumArgs(uint32_t *aNumArgs) override { return _to GetNumArgs(aNumArgs); } \
  NS_IMETHOD GetArg(uint32_t aIndex, nsACString& _retval) override { return _to GetArg(aIndex, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_MOZISANDBOXREPORT(_to) \
  NS_IMETHOD GetMsecAgo(uint64_t *aMsecAgo) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMsecAgo(aMsecAgo); } \
  NS_IMETHOD GetPid(int32_t *aPid) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPid(aPid); } \
  NS_IMETHOD GetTid(int32_t *aTid) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTid(aTid); } \
  NS_IMETHOD GetProcType(nsACString& aProcType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetProcType(aProcType); } \
  NS_IMETHOD GetSyscall(uint32_t *aSyscall) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSyscall(aSyscall); } \
  NS_IMETHOD GetNumArgs(uint32_t *aNumArgs) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNumArgs(aNumArgs); } \
  NS_IMETHOD GetArg(uint32_t aIndex, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetArg(aIndex, _retval); } 


/* starting interface:    mozISandboxReportArray */
#define MOZISANDBOXREPORTARRAY_IID_STR "6e8ff6e5-05c9-42d3-853d-40523fd86a50"

#define MOZISANDBOXREPORTARRAY_IID \
  {0x6e8ff6e5, 0x05c9, 0x42d3, \
    { 0x85, 0x3d, 0x40, 0x52, 0x3f, 0xd8, 0x6a, 0x50 }}

class NS_NO_VTABLE mozISandboxReportArray : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(MOZISANDBOXREPORTARRAY_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = mozISandboxReportArray;

  /* readonly attribute uint64_t begin; */
  NS_IMETHOD GetBegin(uint64_t *aBegin) = 0;

  /* readonly attribute uint64_t end; */
  NS_IMETHOD GetEnd(uint64_t *aEnd) = 0;

  /* mozISandboxReport getElement (in uint64_t aIndex); */
  NS_IMETHOD GetElement(uint64_t aIndex, mozISandboxReport **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(mozISandboxReportArray, MOZISANDBOXREPORTARRAY_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_MOZISANDBOXREPORTARRAY \
  NS_IMETHOD GetBegin(uint64_t *aBegin) override; \
  NS_IMETHOD GetEnd(uint64_t *aEnd) override; \
  NS_IMETHOD GetElement(uint64_t aIndex, mozISandboxReport **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_MOZISANDBOXREPORTARRAY \
  nsresult GetBegin(uint64_t *aBegin); \
  nsresult GetEnd(uint64_t *aEnd); \
  nsresult GetElement(uint64_t aIndex, mozISandboxReport **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_MOZISANDBOXREPORTARRAY(_to) \
  NS_IMETHOD GetBegin(uint64_t *aBegin) override { return _to GetBegin(aBegin); } \
  NS_IMETHOD GetEnd(uint64_t *aEnd) override { return _to GetEnd(aEnd); } \
  NS_IMETHOD GetElement(uint64_t aIndex, mozISandboxReport **_retval) override { return _to GetElement(aIndex, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_MOZISANDBOXREPORTARRAY(_to) \
  NS_IMETHOD GetBegin(uint64_t *aBegin) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetBegin(aBegin); } \
  NS_IMETHOD GetEnd(uint64_t *aEnd) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetEnd(aEnd); } \
  NS_IMETHOD GetElement(uint64_t aIndex, mozISandboxReport **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetElement(aIndex, _retval); } 


/* starting interface:    mozISandboxReporter */
#define MOZISANDBOXREPORTER_IID_STR "8535bdf7-6d9e-4853-acf9-a146449c4a3b"

#define MOZISANDBOXREPORTER_IID \
  {0x8535bdf7, 0x6d9e, 0x4853, \
    { 0xac, 0xf9, 0xa1, 0x46, 0x44, 0x9c, 0x4a, 0x3b }}

class NS_NO_VTABLE mozISandboxReporter : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(MOZISANDBOXREPORTER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = mozISandboxReporter;

  /* mozISandboxReportArray snapshot (); */
  NS_IMETHOD Snapshot(mozISandboxReportArray **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(mozISandboxReporter, MOZISANDBOXREPORTER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_MOZISANDBOXREPORTER \
  NS_IMETHOD Snapshot(mozISandboxReportArray **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_MOZISANDBOXREPORTER \
  nsresult Snapshot(mozISandboxReportArray **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_MOZISANDBOXREPORTER(_to) \
  NS_IMETHOD Snapshot(mozISandboxReportArray **_retval) override { return _to Snapshot(_retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_MOZISANDBOXREPORTER(_to) \
  NS_IMETHOD Snapshot(mozISandboxReportArray **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Snapshot(_retval); } 


#define MOZ_SANDBOX_REPORTER_CID \
{0x5118a6f9, 0x2493, 0x4f97, {0x95, 0x52, 0x62, 0x06, 0x63, 0xe0, 0x3c, 0xb3}}
#define MOZ_SANDBOX_REPORTER_CONTRACTID \
    "@mozilla.org/sandbox/syscall-reporter;1"

#endif /* __gen_mozISandboxReporter_h__ */
