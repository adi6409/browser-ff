/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/ipc/nsIHangReport.idl
 */

#ifndef __gen_nsIHangReport_h__
#define __gen_nsIHangReport_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsFrameLoader; /* webidl FrameLoader */

namespace mozilla {
namespace dom {
class Element; /* webidl Element */
} // namespace dom
} // namespace mozilla


/* starting interface:    nsIHangReport */
#define NS_IHANGREPORT_IID_STR "5fcffbb9-be62-49b1-b8a1-36e820787a74"

#define NS_IHANGREPORT_IID \
  {0x5fcffbb9, 0xbe62, 0x49b1, \
    { 0xb8, 0xa1, 0x36, 0xe8, 0x20, 0x78, 0x7a, 0x74 }}

class NS_NO_VTABLE nsIHangReport : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IHANGREPORT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIHangReport;

  enum {
    SLOW_SCRIPT = 1U,
    PLUGIN_HANG = 2U
  };

  /* readonly attribute unsigned long hangType; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetHangType(uint32_t *aHangType) = 0;

  /* readonly attribute Element scriptBrowser; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetScriptBrowser(mozilla::dom::Element **aScriptBrowser) = 0;

  /* readonly attribute ACString scriptFileName; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetScriptFileName(nsACString& aScriptFileName) = 0;

  /* readonly attribute double hangDuration; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetHangDuration(double *aHangDuration) = 0;

  /* readonly attribute AString addonId; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetAddonId(nsAString& aAddonId) = 0;

  /* readonly attribute ACString pluginName; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPluginName(nsACString& aPluginName) = 0;

  /* readonly attribute unsigned long long childID; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetChildID(uint64_t *aChildID) = 0;

  /* void userCanceled (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD UserCanceled(void) = 0;

  /* void terminateScript (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD TerminateScript(void) = 0;

  /* void terminateGlobal (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD TerminateGlobal(void) = 0;

  /* void terminatePlugin (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD TerminatePlugin(void) = 0;

  /* void beginStartingDebugger (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD BeginStartingDebugger(void) = 0;

  /* void endStartingDebugger (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD EndStartingDebugger(void) = 0;

  /* bool isReportForBrowser (in FrameLoader aFrameLoader); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD IsReportForBrowser(nsFrameLoader *aFrameLoader, bool *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIHangReport, NS_IHANGREPORT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIHANGREPORT \
  NS_IMETHOD GetHangType(uint32_t *aHangType) override; \
  NS_IMETHOD GetScriptBrowser(mozilla::dom::Element **aScriptBrowser) override; \
  NS_IMETHOD GetScriptFileName(nsACString& aScriptFileName) override; \
  NS_IMETHOD GetHangDuration(double *aHangDuration) override; \
  NS_IMETHOD GetAddonId(nsAString& aAddonId) override; \
  NS_IMETHOD GetPluginName(nsACString& aPluginName) override; \
  NS_IMETHOD GetChildID(uint64_t *aChildID) override; \
  NS_IMETHOD UserCanceled(void) override; \
  NS_IMETHOD TerminateScript(void) override; \
  NS_IMETHOD TerminateGlobal(void) override; \
  NS_IMETHOD TerminatePlugin(void) override; \
  NS_IMETHOD BeginStartingDebugger(void) override; \
  NS_IMETHOD EndStartingDebugger(void) override; \
  NS_IMETHOD IsReportForBrowser(nsFrameLoader *aFrameLoader, bool *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIHANGREPORT \
  nsresult GetHangType(uint32_t *aHangType); \
  nsresult GetScriptBrowser(mozilla::dom::Element **aScriptBrowser); \
  nsresult GetScriptFileName(nsACString& aScriptFileName); \
  nsresult GetHangDuration(double *aHangDuration); \
  nsresult GetAddonId(nsAString& aAddonId); \
  nsresult GetPluginName(nsACString& aPluginName); \
  nsresult GetChildID(uint64_t *aChildID); \
  nsresult UserCanceled(void); \
  nsresult TerminateScript(void); \
  nsresult TerminateGlobal(void); \
  nsresult TerminatePlugin(void); \
  nsresult BeginStartingDebugger(void); \
  nsresult EndStartingDebugger(void); \
  nsresult IsReportForBrowser(nsFrameLoader *aFrameLoader, bool *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIHANGREPORT(_to) \
  NS_IMETHOD GetHangType(uint32_t *aHangType) override { return _to GetHangType(aHangType); } \
  NS_IMETHOD GetScriptBrowser(mozilla::dom::Element **aScriptBrowser) override { return _to GetScriptBrowser(aScriptBrowser); } \
  NS_IMETHOD GetScriptFileName(nsACString& aScriptFileName) override { return _to GetScriptFileName(aScriptFileName); } \
  NS_IMETHOD GetHangDuration(double *aHangDuration) override { return _to GetHangDuration(aHangDuration); } \
  NS_IMETHOD GetAddonId(nsAString& aAddonId) override { return _to GetAddonId(aAddonId); } \
  NS_IMETHOD GetPluginName(nsACString& aPluginName) override { return _to GetPluginName(aPluginName); } \
  NS_IMETHOD GetChildID(uint64_t *aChildID) override { return _to GetChildID(aChildID); } \
  NS_IMETHOD UserCanceled(void) override { return _to UserCanceled(); } \
  NS_IMETHOD TerminateScript(void) override { return _to TerminateScript(); } \
  NS_IMETHOD TerminateGlobal(void) override { return _to TerminateGlobal(); } \
  NS_IMETHOD TerminatePlugin(void) override { return _to TerminatePlugin(); } \
  NS_IMETHOD BeginStartingDebugger(void) override { return _to BeginStartingDebugger(); } \
  NS_IMETHOD EndStartingDebugger(void) override { return _to EndStartingDebugger(); } \
  NS_IMETHOD IsReportForBrowser(nsFrameLoader *aFrameLoader, bool *_retval) override { return _to IsReportForBrowser(aFrameLoader, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIHANGREPORT(_to) \
  NS_IMETHOD GetHangType(uint32_t *aHangType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHangType(aHangType); } \
  NS_IMETHOD GetScriptBrowser(mozilla::dom::Element **aScriptBrowser) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetScriptBrowser(aScriptBrowser); } \
  NS_IMETHOD GetScriptFileName(nsACString& aScriptFileName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetScriptFileName(aScriptFileName); } \
  NS_IMETHOD GetHangDuration(double *aHangDuration) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHangDuration(aHangDuration); } \
  NS_IMETHOD GetAddonId(nsAString& aAddonId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAddonId(aAddonId); } \
  NS_IMETHOD GetPluginName(nsACString& aPluginName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPluginName(aPluginName); } \
  NS_IMETHOD GetChildID(uint64_t *aChildID) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetChildID(aChildID); } \
  NS_IMETHOD UserCanceled(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->UserCanceled(); } \
  NS_IMETHOD TerminateScript(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->TerminateScript(); } \
  NS_IMETHOD TerminateGlobal(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->TerminateGlobal(); } \
  NS_IMETHOD TerminatePlugin(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->TerminatePlugin(); } \
  NS_IMETHOD BeginStartingDebugger(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->BeginStartingDebugger(); } \
  NS_IMETHOD EndStartingDebugger(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EndStartingDebugger(); } \
  NS_IMETHOD IsReportForBrowser(nsFrameLoader *aFrameLoader, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsReportForBrowser(aFrameLoader, _retval); } 


#endif /* __gen_nsIHangReport_h__ */
