/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/base/nsIRemoteTab.idl
 */

#ifndef __gen_nsIRemoteTab_h__
#define __gen_nsIRemoteTab_h__


#ifndef __gen_domstubs_h__
#include "domstubs.h"
#endif

#include "js/Value.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIPrincipal; /* forward declaration */

namespace mozilla {
namespace dom {
class Element; /* webidl Element */
} // namespace dom
} // namespace mozilla

namespace mozilla {
namespace dom {
class WindowGlobalParent; /* webidl WindowGlobalParent */
} // namespace dom
} // namespace mozilla


/* starting interface:    nsIRemoteTab */
#define NS_IREMOTETAB_IID_STR "8e49f7b0-1f98-4939-bf91-e9c39cd56434"

#define NS_IREMOTETAB_IID \
  {0x8e49f7b0, 0x1f98, 0x4939, \
    { 0xbf, 0x91, 0xe9, 0xc3, 0x9c, 0xd5, 0x64, 0x34 }}

class NS_NO_VTABLE nsIRemoteTab : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IREMOTETAB_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIRemoteTab;

  /* attribute boolean renderLayers; */
  NS_IMETHOD GetRenderLayers(bool *aRenderLayers) = 0;
  NS_IMETHOD SetRenderLayers(bool aRenderLayers) = 0;

  /* readonly attribute boolean hasLayers; */
  NS_IMETHOD GetHasLayers(bool *aHasLayers) = 0;

  /* void deprioritize (); */
  NS_IMETHOD Deprioritize(void) = 0;

  /* void preserveLayers (in boolean aPreserveLayers); */
  NS_IMETHOD PreserveLayers(bool aPreserveLayers) = 0;

  /* readonly attribute uint64_t tabId; */
  NS_IMETHOD GetTabId(uint64_t *aTabId) = 0;

  /* readonly attribute uint64_t contentProcessId; */
  NS_IMETHOD GetContentProcessId(uint64_t *aContentProcessId) = 0;

  /* readonly attribute int32_t osPid; */
  NS_IMETHOD GetOsPid(int32_t *aOsPid) = 0;

  /* readonly attribute boolean hasPresented; */
  NS_IMETHOD GetHasPresented(bool *aHasPresented) = 0;

  /* void transmitPermissionsForPrincipal (in nsIPrincipal aPrincipal); */
  NS_IMETHOD TransmitPermissionsForPrincipal(nsIPrincipal *aPrincipal) = 0;

  /* boolean startApzAutoscroll (in float aAnchorX, in float aAnchorY, in nsViewID aScrollId, in uint32_t aPresShellId); */
  NS_IMETHOD StartApzAutoscroll(float aAnchorX, float aAnchorY, nsViewID aScrollId, uint32_t aPresShellId, bool *_retval) = 0;

  /* void stopApzAutoscroll (in nsViewID aScrollId, in uint32_t aPresShellId); */
  NS_IMETHOD StopApzAutoscroll(nsViewID aScrollId, uint32_t aPresShellId) = 0;

  enum NavigationType : uint8_t {
    NAVIGATE_BACK = 0,
    NAVIGATE_FORWARD = 1,
    NAVIGATE_INDEX = 2,
    NAVIGATE_URL = 3,
  };

  /* [binaryname(MaybeCancelContentJSExecutionFromScript),implicit_jscontext] void maybeCancelContentJSExecution (in nsIRemoteTab_NavigationType aNavigationType, [optional] in jsval aCancelContentJSOptions); */
  NS_IMETHOD MaybeCancelContentJSExecutionFromScript(nsIRemoteTab::NavigationType aNavigationType, JS::HandleValue aCancelContentJSOptions, JSContext* cx) = 0;

  /* [noscript] void notifyResolutionChanged (); */
  NS_IMETHOD NotifyResolutionChanged(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIRemoteTab, NS_IREMOTETAB_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIREMOTETAB \
  NS_IMETHOD GetRenderLayers(bool *aRenderLayers) override; \
  NS_IMETHOD SetRenderLayers(bool aRenderLayers) override; \
  NS_IMETHOD GetHasLayers(bool *aHasLayers) override; \
  NS_IMETHOD Deprioritize(void) override; \
  NS_IMETHOD PreserveLayers(bool aPreserveLayers) override; \
  NS_IMETHOD GetTabId(uint64_t *aTabId) override; \
  NS_IMETHOD GetContentProcessId(uint64_t *aContentProcessId) override; \
  NS_IMETHOD GetOsPid(int32_t *aOsPid) override; \
  NS_IMETHOD GetHasPresented(bool *aHasPresented) override; \
  NS_IMETHOD TransmitPermissionsForPrincipal(nsIPrincipal *aPrincipal) override; \
  NS_IMETHOD StartApzAutoscroll(float aAnchorX, float aAnchorY, nsViewID aScrollId, uint32_t aPresShellId, bool *_retval) override; \
  NS_IMETHOD StopApzAutoscroll(nsViewID aScrollId, uint32_t aPresShellId) override; \
  NS_IMETHOD MaybeCancelContentJSExecutionFromScript(nsIRemoteTab::NavigationType aNavigationType, JS::HandleValue aCancelContentJSOptions, JSContext* cx) override; \
  NS_IMETHOD NotifyResolutionChanged(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIREMOTETAB \
  nsresult GetRenderLayers(bool *aRenderLayers); \
  nsresult SetRenderLayers(bool aRenderLayers); \
  nsresult GetHasLayers(bool *aHasLayers); \
  nsresult Deprioritize(void); \
  nsresult PreserveLayers(bool aPreserveLayers); \
  nsresult GetTabId(uint64_t *aTabId); \
  nsresult GetContentProcessId(uint64_t *aContentProcessId); \
  nsresult GetOsPid(int32_t *aOsPid); \
  nsresult GetHasPresented(bool *aHasPresented); \
  nsresult TransmitPermissionsForPrincipal(nsIPrincipal *aPrincipal); \
  nsresult StartApzAutoscroll(float aAnchorX, float aAnchorY, nsViewID aScrollId, uint32_t aPresShellId, bool *_retval); \
  nsresult StopApzAutoscroll(nsViewID aScrollId, uint32_t aPresShellId); \
  nsresult MaybeCancelContentJSExecutionFromScript(nsIRemoteTab::NavigationType aNavigationType, JS::HandleValue aCancelContentJSOptions, JSContext* cx); \
  nsresult NotifyResolutionChanged(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIREMOTETAB(_to) \
  NS_IMETHOD GetRenderLayers(bool *aRenderLayers) override { return _to GetRenderLayers(aRenderLayers); } \
  NS_IMETHOD SetRenderLayers(bool aRenderLayers) override { return _to SetRenderLayers(aRenderLayers); } \
  NS_IMETHOD GetHasLayers(bool *aHasLayers) override { return _to GetHasLayers(aHasLayers); } \
  NS_IMETHOD Deprioritize(void) override { return _to Deprioritize(); } \
  NS_IMETHOD PreserveLayers(bool aPreserveLayers) override { return _to PreserveLayers(aPreserveLayers); } \
  NS_IMETHOD GetTabId(uint64_t *aTabId) override { return _to GetTabId(aTabId); } \
  NS_IMETHOD GetContentProcessId(uint64_t *aContentProcessId) override { return _to GetContentProcessId(aContentProcessId); } \
  NS_IMETHOD GetOsPid(int32_t *aOsPid) override { return _to GetOsPid(aOsPid); } \
  NS_IMETHOD GetHasPresented(bool *aHasPresented) override { return _to GetHasPresented(aHasPresented); } \
  NS_IMETHOD TransmitPermissionsForPrincipal(nsIPrincipal *aPrincipal) override { return _to TransmitPermissionsForPrincipal(aPrincipal); } \
  NS_IMETHOD StartApzAutoscroll(float aAnchorX, float aAnchorY, nsViewID aScrollId, uint32_t aPresShellId, bool *_retval) override { return _to StartApzAutoscroll(aAnchorX, aAnchorY, aScrollId, aPresShellId, _retval); } \
  NS_IMETHOD StopApzAutoscroll(nsViewID aScrollId, uint32_t aPresShellId) override { return _to StopApzAutoscroll(aScrollId, aPresShellId); } \
  NS_IMETHOD MaybeCancelContentJSExecutionFromScript(nsIRemoteTab::NavigationType aNavigationType, JS::HandleValue aCancelContentJSOptions, JSContext* cx) override { return _to MaybeCancelContentJSExecutionFromScript(aNavigationType, aCancelContentJSOptions, cx); } \
  NS_IMETHOD NotifyResolutionChanged(void) override { return _to NotifyResolutionChanged(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIREMOTETAB(_to) \
  NS_IMETHOD GetRenderLayers(bool *aRenderLayers) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRenderLayers(aRenderLayers); } \
  NS_IMETHOD SetRenderLayers(bool aRenderLayers) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetRenderLayers(aRenderLayers); } \
  NS_IMETHOD GetHasLayers(bool *aHasLayers) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHasLayers(aHasLayers); } \
  NS_IMETHOD Deprioritize(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Deprioritize(); } \
  NS_IMETHOD PreserveLayers(bool aPreserveLayers) override { return !_to ? NS_ERROR_NULL_POINTER : _to->PreserveLayers(aPreserveLayers); } \
  NS_IMETHOD GetTabId(uint64_t *aTabId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTabId(aTabId); } \
  NS_IMETHOD GetContentProcessId(uint64_t *aContentProcessId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetContentProcessId(aContentProcessId); } \
  NS_IMETHOD GetOsPid(int32_t *aOsPid) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOsPid(aOsPid); } \
  NS_IMETHOD GetHasPresented(bool *aHasPresented) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHasPresented(aHasPresented); } \
  NS_IMETHOD TransmitPermissionsForPrincipal(nsIPrincipal *aPrincipal) override { return !_to ? NS_ERROR_NULL_POINTER : _to->TransmitPermissionsForPrincipal(aPrincipal); } \
  NS_IMETHOD StartApzAutoscroll(float aAnchorX, float aAnchorY, nsViewID aScrollId, uint32_t aPresShellId, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->StartApzAutoscroll(aAnchorX, aAnchorY, aScrollId, aPresShellId, _retval); } \
  NS_IMETHOD StopApzAutoscroll(nsViewID aScrollId, uint32_t aPresShellId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->StopApzAutoscroll(aScrollId, aPresShellId); } \
  NS_IMETHOD MaybeCancelContentJSExecutionFromScript(nsIRemoteTab::NavigationType aNavigationType, JS::HandleValue aCancelContentJSOptions, JSContext* cx) override { return !_to ? NS_ERROR_NULL_POINTER : _to->MaybeCancelContentJSExecutionFromScript(aNavigationType, aCancelContentJSOptions, cx); } \
  NS_IMETHOD NotifyResolutionChanged(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NotifyResolutionChanged(); } 


#endif /* __gen_nsIRemoteTab_h__ */
