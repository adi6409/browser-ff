/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsIDragService.idl
 */

#ifndef __gen_nsIDragService_h__
#define __gen_nsIDragService_h__


#ifndef __gen_nsIArray_h__
#include "nsIArray.h"
#endif

#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsIDragSession_h__
#include "nsIDragSession.h"
#endif

#ifndef __gen_nsIContentPolicy_h__
#include "nsIContentPolicy.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
namespace mozilla {
namespace dom {
class DragEvent; /* webidl DragEvent */
} // namespace dom
} // namespace mozilla

class nsINode; /* webidl Node */

namespace mozilla {
namespace dom {
class Selection; /* webidl Selection */
} // namespace dom
} // namespace mozilla

class nsICookieJarSettings; /* forward declaration */

#include "mozilla/EventForwards.h"
namespace mozilla {
namespace dom {
class ContentParent;
class DataTransfer;
class RemoteDragStartData;
} // namespace dom
} // namespace mozilla

/* starting interface:    nsIDragService */
#define NS_IDRAGSERVICE_IID_STR "ebd6b3a2-af16-43af-a698-3091a087dd62"

#define NS_IDRAGSERVICE_IID \
  {0xebd6b3a2, 0xaf16, 0x43af, \
    { 0xa6, 0x98, 0x30, 0x91, 0xa0, 0x87, 0xdd, 0x62 }}

class NS_NO_VTABLE nsIDragService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IDRAGSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIDragService;

  enum {
    DRAGDROP_ACTION_NONE = 0,
    DRAGDROP_ACTION_COPY = 1,
    DRAGDROP_ACTION_MOVE = 2,
    DRAGDROP_ACTION_LINK = 4,
    DRAGDROP_ACTION_UNINITIALIZED = 64
  };

  /* [can_run_script] void invokeDragSession (in Node aDOMNode, in nsIPrincipal aPrincipal, in nsIContentSecurityPolicy aCsp, in nsICookieJarSettings aCookieJarSettings, in nsIArray aTransferables, in unsigned long aActionType, [optional] in nsContentPolicyType aContentPolicyType); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD InvokeDragSession(nsINode *aDOMNode, nsIPrincipal *aPrincipal, nsIContentSecurityPolicy *aCsp, nsICookieJarSettings *aCookieJarSettings, nsIArray *aTransferables, uint32_t aActionType, nsContentPolicyType aContentPolicyType) = 0;

  /* [can_run_script,noscript] void invokeDragSessionWithImage (in Node aDOMNode, in nsIPrincipal aPrincipal, in nsIContentSecurityPolicy aCsp, in nsICookieJarSettings aCookieJarSettings, in nsIArray aTransferableArray, in unsigned long aActionType, in Node aImage, in long aImageX, in long aImageY, in DragEvent aDragEvent, in DataTransferPtr aDataTransfer); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD InvokeDragSessionWithImage(nsINode *aDOMNode, nsIPrincipal *aPrincipal, nsIContentSecurityPolicy *aCsp, nsICookieJarSettings *aCookieJarSettings, nsIArray *aTransferableArray, uint32_t aActionType, nsINode *aImage, int32_t aImageX, int32_t aImageY, mozilla::dom::DragEvent *aDragEvent, mozilla::dom::DataTransfer * aDataTransfer) = 0;

  /* [can_run_script,noscript] void invokeDragSessionWithRemoteImage (in Node aDOMNode, in nsIPrincipal aPrincipal, in nsIContentSecurityPolicy aCsp, in nsICookieJarSettings aCookieJarSettings, in nsIArray aTransferableArray, in unsigned long aActionType, in RemoteDragStartDataPtr aDragStartData, in DragEvent aDragEvent, in DataTransferPtr aDataTransfer); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD InvokeDragSessionWithRemoteImage(nsINode *aDOMNode, nsIPrincipal *aPrincipal, nsIContentSecurityPolicy *aCsp, nsICookieJarSettings *aCookieJarSettings, nsIArray *aTransferableArray, uint32_t aActionType, mozilla::dom::RemoteDragStartData * aDragStartData, mozilla::dom::DragEvent *aDragEvent, mozilla::dom::DataTransfer * aDataTransfer) = 0;

  /* [can_run_script] void invokeDragSessionWithSelection (in Selection aSelection, in nsIPrincipal aPrincipal, in nsIContentSecurityPolicy aCsp, in nsICookieJarSettings aCookieJarSettings, in nsIArray aTransferableArray, in unsigned long aActionType, in DragEvent aDragEvent, in DataTransferPtr aDataTransfer); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD InvokeDragSessionWithSelection(mozilla::dom::Selection *aSelection, nsIPrincipal *aPrincipal, nsIContentSecurityPolicy *aCsp, nsICookieJarSettings *aCookieJarSettings, nsIArray *aTransferableArray, uint32_t aActionType, mozilla::dom::DragEvent *aDragEvent, mozilla::dom::DataTransfer * aDataTransfer) = 0;

  /* nsIDragSession getCurrentSession (); */
  NS_IMETHOD GetCurrentSession(nsIDragSession **_retval) = 0;

  /* void startDragSession (); */
  NS_IMETHOD StartDragSession(void) = 0;

  /* void startDragSessionForTests (in unsigned long aAllowedEffect); */
  NS_IMETHOD StartDragSessionForTests(uint32_t aAllowedEffect) = 0;

  /* [can_run_script] void endDragSession (in boolean aDoneDrag, [optional] in unsigned long aKeyModifiers); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD EndDragSession(bool aDoneDrag, uint32_t aKeyModifiers) = 0;

  /* [can_run_script,noscript] void fireDragEventAtSource (in EventMessage aEventMessage, in unsigned long aKeyModifiers); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD FireDragEventAtSource(mozilla::EventMessage aEventMessage, uint32_t aKeyModifiers) = 0;

  /* [can_run_script] void suppress (); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD Suppress(void) = 0;

  /* void unsuppress (); */
  NS_IMETHOD Unsuppress(void) = 0;

  /* [noscript] void dragMoved (in long aX, in long aY); */
  NS_IMETHOD DragMoved(int32_t aX, int32_t aY) = 0;

  /* [nostdcall,notxpcom] boolean maybeAddChildProcess (in ContentParentPtr aChild); */
  virtual bool MaybeAddChildProcess(mozilla::dom::ContentParent * aChild) = 0;

  /* [nostdcall,notxpcom] boolean removeAllChildProcesses (); */
  virtual bool RemoveAllChildProcesses(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIDragService, NS_IDRAGSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIDRAGSERVICE \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD InvokeDragSession(nsINode *aDOMNode, nsIPrincipal *aPrincipal, nsIContentSecurityPolicy *aCsp, nsICookieJarSettings *aCookieJarSettings, nsIArray *aTransferables, uint32_t aActionType, nsContentPolicyType aContentPolicyType) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD InvokeDragSessionWithImage(nsINode *aDOMNode, nsIPrincipal *aPrincipal, nsIContentSecurityPolicy *aCsp, nsICookieJarSettings *aCookieJarSettings, nsIArray *aTransferableArray, uint32_t aActionType, nsINode *aImage, int32_t aImageX, int32_t aImageY, mozilla::dom::DragEvent *aDragEvent, mozilla::dom::DataTransfer * aDataTransfer) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD InvokeDragSessionWithRemoteImage(nsINode *aDOMNode, nsIPrincipal *aPrincipal, nsIContentSecurityPolicy *aCsp, nsICookieJarSettings *aCookieJarSettings, nsIArray *aTransferableArray, uint32_t aActionType, mozilla::dom::RemoteDragStartData * aDragStartData, mozilla::dom::DragEvent *aDragEvent, mozilla::dom::DataTransfer * aDataTransfer) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD InvokeDragSessionWithSelection(mozilla::dom::Selection *aSelection, nsIPrincipal *aPrincipal, nsIContentSecurityPolicy *aCsp, nsICookieJarSettings *aCookieJarSettings, nsIArray *aTransferableArray, uint32_t aActionType, mozilla::dom::DragEvent *aDragEvent, mozilla::dom::DataTransfer * aDataTransfer) override; \
  NS_IMETHOD GetCurrentSession(nsIDragSession **_retval) override; \
  NS_IMETHOD StartDragSession(void) override; \
  NS_IMETHOD StartDragSessionForTests(uint32_t aAllowedEffect) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD EndDragSession(bool aDoneDrag, uint32_t aKeyModifiers) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD FireDragEventAtSource(mozilla::EventMessage aEventMessage, uint32_t aKeyModifiers) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD Suppress(void) override; \
  NS_IMETHOD Unsuppress(void) override; \
  NS_IMETHOD DragMoved(int32_t aX, int32_t aY) override; \
  virtual bool MaybeAddChildProcess(mozilla::dom::ContentParent * aChild) override; \
  virtual bool RemoveAllChildProcesses(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIDRAGSERVICE \
  MOZ_CAN_RUN_SCRIPT nsresult InvokeDragSession(nsINode *aDOMNode, nsIPrincipal *aPrincipal, nsIContentSecurityPolicy *aCsp, nsICookieJarSettings *aCookieJarSettings, nsIArray *aTransferables, uint32_t aActionType, nsContentPolicyType aContentPolicyType); \
  MOZ_CAN_RUN_SCRIPT nsresult InvokeDragSessionWithImage(nsINode *aDOMNode, nsIPrincipal *aPrincipal, nsIContentSecurityPolicy *aCsp, nsICookieJarSettings *aCookieJarSettings, nsIArray *aTransferableArray, uint32_t aActionType, nsINode *aImage, int32_t aImageX, int32_t aImageY, mozilla::dom::DragEvent *aDragEvent, mozilla::dom::DataTransfer * aDataTransfer); \
  MOZ_CAN_RUN_SCRIPT nsresult InvokeDragSessionWithRemoteImage(nsINode *aDOMNode, nsIPrincipal *aPrincipal, nsIContentSecurityPolicy *aCsp, nsICookieJarSettings *aCookieJarSettings, nsIArray *aTransferableArray, uint32_t aActionType, mozilla::dom::RemoteDragStartData * aDragStartData, mozilla::dom::DragEvent *aDragEvent, mozilla::dom::DataTransfer * aDataTransfer); \
  MOZ_CAN_RUN_SCRIPT nsresult InvokeDragSessionWithSelection(mozilla::dom::Selection *aSelection, nsIPrincipal *aPrincipal, nsIContentSecurityPolicy *aCsp, nsICookieJarSettings *aCookieJarSettings, nsIArray *aTransferableArray, uint32_t aActionType, mozilla::dom::DragEvent *aDragEvent, mozilla::dom::DataTransfer * aDataTransfer); \
  nsresult GetCurrentSession(nsIDragSession **_retval); \
  nsresult StartDragSession(void); \
  nsresult StartDragSessionForTests(uint32_t aAllowedEffect); \
  MOZ_CAN_RUN_SCRIPT nsresult EndDragSession(bool aDoneDrag, uint32_t aKeyModifiers); \
  MOZ_CAN_RUN_SCRIPT nsresult FireDragEventAtSource(mozilla::EventMessage aEventMessage, uint32_t aKeyModifiers); \
  MOZ_CAN_RUN_SCRIPT nsresult Suppress(void); \
  nsresult Unsuppress(void); \
  nsresult DragMoved(int32_t aX, int32_t aY); \
  bool MaybeAddChildProcess(mozilla::dom::ContentParent * aChild); \
  bool RemoveAllChildProcesses(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIDRAGSERVICE(_to) \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD InvokeDragSession(nsINode *aDOMNode, nsIPrincipal *aPrincipal, nsIContentSecurityPolicy *aCsp, nsICookieJarSettings *aCookieJarSettings, nsIArray *aTransferables, uint32_t aActionType, nsContentPolicyType aContentPolicyType) override { return _to InvokeDragSession(aDOMNode, aPrincipal, aCsp, aCookieJarSettings, aTransferables, aActionType, aContentPolicyType); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD InvokeDragSessionWithImage(nsINode *aDOMNode, nsIPrincipal *aPrincipal, nsIContentSecurityPolicy *aCsp, nsICookieJarSettings *aCookieJarSettings, nsIArray *aTransferableArray, uint32_t aActionType, nsINode *aImage, int32_t aImageX, int32_t aImageY, mozilla::dom::DragEvent *aDragEvent, mozilla::dom::DataTransfer * aDataTransfer) override { return _to InvokeDragSessionWithImage(aDOMNode, aPrincipal, aCsp, aCookieJarSettings, aTransferableArray, aActionType, aImage, aImageX, aImageY, aDragEvent, aDataTransfer); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD InvokeDragSessionWithRemoteImage(nsINode *aDOMNode, nsIPrincipal *aPrincipal, nsIContentSecurityPolicy *aCsp, nsICookieJarSettings *aCookieJarSettings, nsIArray *aTransferableArray, uint32_t aActionType, mozilla::dom::RemoteDragStartData * aDragStartData, mozilla::dom::DragEvent *aDragEvent, mozilla::dom::DataTransfer * aDataTransfer) override { return _to InvokeDragSessionWithRemoteImage(aDOMNode, aPrincipal, aCsp, aCookieJarSettings, aTransferableArray, aActionType, aDragStartData, aDragEvent, aDataTransfer); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD InvokeDragSessionWithSelection(mozilla::dom::Selection *aSelection, nsIPrincipal *aPrincipal, nsIContentSecurityPolicy *aCsp, nsICookieJarSettings *aCookieJarSettings, nsIArray *aTransferableArray, uint32_t aActionType, mozilla::dom::DragEvent *aDragEvent, mozilla::dom::DataTransfer * aDataTransfer) override { return _to InvokeDragSessionWithSelection(aSelection, aPrincipal, aCsp, aCookieJarSettings, aTransferableArray, aActionType, aDragEvent, aDataTransfer); } \
  NS_IMETHOD GetCurrentSession(nsIDragSession **_retval) override { return _to GetCurrentSession(_retval); } \
  NS_IMETHOD StartDragSession(void) override { return _to StartDragSession(); } \
  NS_IMETHOD StartDragSessionForTests(uint32_t aAllowedEffect) override { return _to StartDragSessionForTests(aAllowedEffect); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD EndDragSession(bool aDoneDrag, uint32_t aKeyModifiers) override { return _to EndDragSession(aDoneDrag, aKeyModifiers); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD FireDragEventAtSource(mozilla::EventMessage aEventMessage, uint32_t aKeyModifiers) override { return _to FireDragEventAtSource(aEventMessage, aKeyModifiers); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD Suppress(void) override { return _to Suppress(); } \
  NS_IMETHOD Unsuppress(void) override { return _to Unsuppress(); } \
  NS_IMETHOD DragMoved(int32_t aX, int32_t aY) override { return _to DragMoved(aX, aY); } \
  virtual bool MaybeAddChildProcess(mozilla::dom::ContentParent * aChild) override { return _to MaybeAddChildProcess(aChild); } \
  virtual bool RemoveAllChildProcesses(void) override { return _to RemoveAllChildProcesses(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIDRAGSERVICE(_to) \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD InvokeDragSession(nsINode *aDOMNode, nsIPrincipal *aPrincipal, nsIContentSecurityPolicy *aCsp, nsICookieJarSettings *aCookieJarSettings, nsIArray *aTransferables, uint32_t aActionType, nsContentPolicyType aContentPolicyType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InvokeDragSession(aDOMNode, aPrincipal, aCsp, aCookieJarSettings, aTransferables, aActionType, aContentPolicyType); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD InvokeDragSessionWithImage(nsINode *aDOMNode, nsIPrincipal *aPrincipal, nsIContentSecurityPolicy *aCsp, nsICookieJarSettings *aCookieJarSettings, nsIArray *aTransferableArray, uint32_t aActionType, nsINode *aImage, int32_t aImageX, int32_t aImageY, mozilla::dom::DragEvent *aDragEvent, mozilla::dom::DataTransfer * aDataTransfer) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InvokeDragSessionWithImage(aDOMNode, aPrincipal, aCsp, aCookieJarSettings, aTransferableArray, aActionType, aImage, aImageX, aImageY, aDragEvent, aDataTransfer); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD InvokeDragSessionWithRemoteImage(nsINode *aDOMNode, nsIPrincipal *aPrincipal, nsIContentSecurityPolicy *aCsp, nsICookieJarSettings *aCookieJarSettings, nsIArray *aTransferableArray, uint32_t aActionType, mozilla::dom::RemoteDragStartData * aDragStartData, mozilla::dom::DragEvent *aDragEvent, mozilla::dom::DataTransfer * aDataTransfer) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InvokeDragSessionWithRemoteImage(aDOMNode, aPrincipal, aCsp, aCookieJarSettings, aTransferableArray, aActionType, aDragStartData, aDragEvent, aDataTransfer); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD InvokeDragSessionWithSelection(mozilla::dom::Selection *aSelection, nsIPrincipal *aPrincipal, nsIContentSecurityPolicy *aCsp, nsICookieJarSettings *aCookieJarSettings, nsIArray *aTransferableArray, uint32_t aActionType, mozilla::dom::DragEvent *aDragEvent, mozilla::dom::DataTransfer * aDataTransfer) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InvokeDragSessionWithSelection(aSelection, aPrincipal, aCsp, aCookieJarSettings, aTransferableArray, aActionType, aDragEvent, aDataTransfer); } \
  NS_IMETHOD GetCurrentSession(nsIDragSession **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCurrentSession(_retval); } \
  NS_IMETHOD StartDragSession(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->StartDragSession(); } \
  NS_IMETHOD StartDragSessionForTests(uint32_t aAllowedEffect) override { return !_to ? NS_ERROR_NULL_POINTER : _to->StartDragSessionForTests(aAllowedEffect); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD EndDragSession(bool aDoneDrag, uint32_t aKeyModifiers) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EndDragSession(aDoneDrag, aKeyModifiers); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD FireDragEventAtSource(mozilla::EventMessage aEventMessage, uint32_t aKeyModifiers) override { return !_to ? NS_ERROR_NULL_POINTER : _to->FireDragEventAtSource(aEventMessage, aKeyModifiers); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD Suppress(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Suppress(); } \
  NS_IMETHOD Unsuppress(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Unsuppress(); } \
  NS_IMETHOD DragMoved(int32_t aX, int32_t aY) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DragMoved(aX, aY); } \
  virtual bool MaybeAddChildProcess(mozilla::dom::ContentParent * aChild) override; \
  virtual bool RemoveAllChildProcesses(void) override; 



#endif /* __gen_nsIDragService_h__ */
