/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsIDragSession.idl
 */

#ifndef __gen_nsIDragSession_h__
#define __gen_nsIDragSession_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsITransferable_h__
#include "nsITransferable.h"
#endif

#include "mozilla/AlreadyAddRefed.h"
#include "mozilla/Assertions.h"
#include "mozilla/DebugOnly.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
#include "nsSize.h"
class nsIContentSecurityPolicy; /* forward declaration */

namespace mozilla {
namespace dom {
class DataTransfer; /* webidl DataTransfer */
} // namespace dom
} // namespace mozilla

namespace mozilla {
namespace dom {
class Document; /* webidl Document */
} // namespace dom
} // namespace mozilla

class nsINode; /* webidl Node */


/* starting interface:    nsIDragSession */
#define NS_IDRAGSESSION_IID_STR "25bce737-73f0-43c7-bc20-c71044a73c5a"

#define NS_IDRAGSESSION_IID \
  {0x25bce737, 0x73f0, 0x43c7, \
    { 0xbc, 0x20, 0xc7, 0x10, 0x44, 0xa7, 0x3c, 0x5a }}

class NS_NO_VTABLE nsIDragSession : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IDRAGSESSION_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIDragSession;

  /* attribute boolean canDrop; */
  NS_IMETHOD GetCanDrop(bool *aCanDrop) = 0;
  NS_IMETHOD SetCanDrop(bool aCanDrop) = 0;

  /* attribute boolean onlyChromeDrop; */
  NS_IMETHOD GetOnlyChromeDrop(bool *aOnlyChromeDrop) = 0;
  NS_IMETHOD SetOnlyChromeDrop(bool aOnlyChromeDrop) = 0;

  /* attribute unsigned long dragAction; */
  NS_IMETHOD GetDragAction(uint32_t *aDragAction) = 0;
  NS_IMETHOD SetDragAction(uint32_t aDragAction) = 0;

  /* readonly attribute unsigned long numDropItems; */
  NS_IMETHOD GetNumDropItems(uint32_t *aNumDropItems) = 0;

  /* [infallible] readonly attribute Document sourceDocument; */
  NS_IMETHOD GetSourceDocument(mozilla::dom::Document **aSourceDocument) = 0;
   inline already_AddRefed<mozilla::dom::Document> GetSourceDocument()
  {
    mozilla::dom::Document* result = nullptr;
    mozilla::DebugOnly<nsresult> rv = GetSourceDocument(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return already_AddRefed<mozilla::dom::Document>(result);
  }

  /* readonly attribute Node sourceNode; */
  NS_IMETHOD GetSourceNode(nsINode **aSourceNode) = 0;

  /* attribute nsIPrincipal triggeringPrincipal; */
  NS_IMETHOD GetTriggeringPrincipal(nsIPrincipal **aTriggeringPrincipal) = 0;
  NS_IMETHOD SetTriggeringPrincipal(nsIPrincipal *aTriggeringPrincipal) = 0;

  /* attribute nsIContentSecurityPolicy csp; */
  NS_IMETHOD GetCsp(nsIContentSecurityPolicy **aCsp) = 0;
  NS_IMETHOD SetCsp(nsIContentSecurityPolicy *aCsp) = 0;

  /* [binaryname(DataTransferXPCOM)] attribute DataTransfer dataTransfer; */
  NS_IMETHOD GetDataTransferXPCOM(mozilla::dom::DataTransfer **aDataTransfer) = 0;
  NS_IMETHOD SetDataTransferXPCOM(mozilla::dom::DataTransfer *aDataTransfer) = 0;

  /* [nostdcall,notxpcom] DataTransfer getDataTransfer (); */
  virtual mozilla::dom::DataTransfer * GetDataTransfer(void) = 0;

  /* [nostdcall,notxpcom] void setDataTransfer (in DataTransfer aDataTransfer); */
  virtual void SetDataTransfer(mozilla::dom::DataTransfer *aDataTransfer) = 0;

  /* void getData (in nsITransferable aTransferable, in unsigned long aItemIndex); */
  NS_IMETHOD GetData(nsITransferable *aTransferable, uint32_t aItemIndex) = 0;

  /* boolean isDataFlavorSupported (in string aDataFlavor); */
  NS_IMETHOD IsDataFlavorSupported(const char * aDataFlavor, bool *_retval) = 0;

  /* void userCancelled (); */
  NS_IMETHOD UserCancelled(void) = 0;

  /* void dragEventDispatchedToChildProcess (); */
  NS_IMETHOD DragEventDispatchedToChildProcess(void) = 0;

  /* void updateDragEffect (); */
  NS_IMETHOD UpdateDragEffect(void) = 0;

  /* void updateDragImage (in Node aImage, in long aImageX, in long aImageY); */
  NS_IMETHOD UpdateDragImage(nsINode *aImage, int32_t aImageX, int32_t aImageY) = 0;

  /* [nostdcall,notxpcom] unsigned long getEffectAllowedForTests (); */
  virtual uint32_t GetEffectAllowedForTests(void) = 0;

  /* [nostdcall,notxpcom] bool isSynthesizedForTests (); */
  virtual bool IsSynthesizedForTests(void) = 0;

  /* void setDragEndPointForTests (in long aScreenX, in long aScreenY); */
  NS_IMETHOD SetDragEndPointForTests(int32_t aScreenX, int32_t aScreenY) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIDragSession, NS_IDRAGSESSION_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIDRAGSESSION \
  NS_IMETHOD GetCanDrop(bool *aCanDrop) override; \
  NS_IMETHOD SetCanDrop(bool aCanDrop) override; \
  NS_IMETHOD GetOnlyChromeDrop(bool *aOnlyChromeDrop) override; \
  NS_IMETHOD SetOnlyChromeDrop(bool aOnlyChromeDrop) override; \
  NS_IMETHOD GetDragAction(uint32_t *aDragAction) override; \
  NS_IMETHOD SetDragAction(uint32_t aDragAction) override; \
  NS_IMETHOD GetNumDropItems(uint32_t *aNumDropItems) override; \
  using nsIDragSession::GetSourceDocument; \
  NS_IMETHOD GetSourceDocument(mozilla::dom::Document **aSourceDocument) override; \
  NS_IMETHOD GetSourceNode(nsINode **aSourceNode) override; \
  NS_IMETHOD GetTriggeringPrincipal(nsIPrincipal **aTriggeringPrincipal) override; \
  NS_IMETHOD SetTriggeringPrincipal(nsIPrincipal *aTriggeringPrincipal) override; \
  NS_IMETHOD GetCsp(nsIContentSecurityPolicy **aCsp) override; \
  NS_IMETHOD SetCsp(nsIContentSecurityPolicy *aCsp) override; \
  NS_IMETHOD GetDataTransferXPCOM(mozilla::dom::DataTransfer **aDataTransfer) override; \
  NS_IMETHOD SetDataTransferXPCOM(mozilla::dom::DataTransfer *aDataTransfer) override; \
  virtual mozilla::dom::DataTransfer * GetDataTransfer(void) override; \
  virtual void SetDataTransfer(mozilla::dom::DataTransfer *aDataTransfer) override; \
  NS_IMETHOD GetData(nsITransferable *aTransferable, uint32_t aItemIndex) override; \
  NS_IMETHOD IsDataFlavorSupported(const char * aDataFlavor, bool *_retval) override; \
  NS_IMETHOD UserCancelled(void) override; \
  NS_IMETHOD DragEventDispatchedToChildProcess(void) override; \
  NS_IMETHOD UpdateDragEffect(void) override; \
  NS_IMETHOD UpdateDragImage(nsINode *aImage, int32_t aImageX, int32_t aImageY) override; \
  virtual uint32_t GetEffectAllowedForTests(void) override; \
  virtual bool IsSynthesizedForTests(void) override; \
  NS_IMETHOD SetDragEndPointForTests(int32_t aScreenX, int32_t aScreenY) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIDRAGSESSION \
  nsresult GetCanDrop(bool *aCanDrop); \
  nsresult SetCanDrop(bool aCanDrop); \
  nsresult GetOnlyChromeDrop(bool *aOnlyChromeDrop); \
  nsresult SetOnlyChromeDrop(bool aOnlyChromeDrop); \
  nsresult GetDragAction(uint32_t *aDragAction); \
  nsresult SetDragAction(uint32_t aDragAction); \
  nsresult GetNumDropItems(uint32_t *aNumDropItems); \
  using nsIDragSession::GetSourceDocument; \
  nsresult GetSourceDocument(mozilla::dom::Document **aSourceDocument); \
  nsresult GetSourceNode(nsINode **aSourceNode); \
  nsresult GetTriggeringPrincipal(nsIPrincipal **aTriggeringPrincipal); \
  nsresult SetTriggeringPrincipal(nsIPrincipal *aTriggeringPrincipal); \
  nsresult GetCsp(nsIContentSecurityPolicy **aCsp); \
  nsresult SetCsp(nsIContentSecurityPolicy *aCsp); \
  nsresult GetDataTransferXPCOM(mozilla::dom::DataTransfer **aDataTransfer); \
  nsresult SetDataTransferXPCOM(mozilla::dom::DataTransfer *aDataTransfer); \
  mozilla::dom::DataTransfer * GetDataTransfer(void); \
  void SetDataTransfer(mozilla::dom::DataTransfer *aDataTransfer); \
  nsresult GetData(nsITransferable *aTransferable, uint32_t aItemIndex); \
  nsresult IsDataFlavorSupported(const char * aDataFlavor, bool *_retval); \
  nsresult UserCancelled(void); \
  nsresult DragEventDispatchedToChildProcess(void); \
  nsresult UpdateDragEffect(void); \
  nsresult UpdateDragImage(nsINode *aImage, int32_t aImageX, int32_t aImageY); \
  uint32_t GetEffectAllowedForTests(void); \
  bool IsSynthesizedForTests(void); \
  nsresult SetDragEndPointForTests(int32_t aScreenX, int32_t aScreenY); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIDRAGSESSION(_to) \
  NS_IMETHOD GetCanDrop(bool *aCanDrop) override { return _to GetCanDrop(aCanDrop); } \
  NS_IMETHOD SetCanDrop(bool aCanDrop) override { return _to SetCanDrop(aCanDrop); } \
  NS_IMETHOD GetOnlyChromeDrop(bool *aOnlyChromeDrop) override { return _to GetOnlyChromeDrop(aOnlyChromeDrop); } \
  NS_IMETHOD SetOnlyChromeDrop(bool aOnlyChromeDrop) override { return _to SetOnlyChromeDrop(aOnlyChromeDrop); } \
  NS_IMETHOD GetDragAction(uint32_t *aDragAction) override { return _to GetDragAction(aDragAction); } \
  NS_IMETHOD SetDragAction(uint32_t aDragAction) override { return _to SetDragAction(aDragAction); } \
  NS_IMETHOD GetNumDropItems(uint32_t *aNumDropItems) override { return _to GetNumDropItems(aNumDropItems); } \
  using nsIDragSession::GetSourceDocument; \
  NS_IMETHOD GetSourceDocument(mozilla::dom::Document **aSourceDocument) override { return _to GetSourceDocument(aSourceDocument); } \
  NS_IMETHOD GetSourceNode(nsINode **aSourceNode) override { return _to GetSourceNode(aSourceNode); } \
  NS_IMETHOD GetTriggeringPrincipal(nsIPrincipal **aTriggeringPrincipal) override { return _to GetTriggeringPrincipal(aTriggeringPrincipal); } \
  NS_IMETHOD SetTriggeringPrincipal(nsIPrincipal *aTriggeringPrincipal) override { return _to SetTriggeringPrincipal(aTriggeringPrincipal); } \
  NS_IMETHOD GetCsp(nsIContentSecurityPolicy **aCsp) override { return _to GetCsp(aCsp); } \
  NS_IMETHOD SetCsp(nsIContentSecurityPolicy *aCsp) override { return _to SetCsp(aCsp); } \
  NS_IMETHOD GetDataTransferXPCOM(mozilla::dom::DataTransfer **aDataTransfer) override { return _to GetDataTransferXPCOM(aDataTransfer); } \
  NS_IMETHOD SetDataTransferXPCOM(mozilla::dom::DataTransfer *aDataTransfer) override { return _to SetDataTransferXPCOM(aDataTransfer); } \
  virtual mozilla::dom::DataTransfer * GetDataTransfer(void) override { return _to GetDataTransfer(); } \
  virtual void SetDataTransfer(mozilla::dom::DataTransfer *aDataTransfer) override { return _to SetDataTransfer(aDataTransfer); } \
  NS_IMETHOD GetData(nsITransferable *aTransferable, uint32_t aItemIndex) override { return _to GetData(aTransferable, aItemIndex); } \
  NS_IMETHOD IsDataFlavorSupported(const char * aDataFlavor, bool *_retval) override { return _to IsDataFlavorSupported(aDataFlavor, _retval); } \
  NS_IMETHOD UserCancelled(void) override { return _to UserCancelled(); } \
  NS_IMETHOD DragEventDispatchedToChildProcess(void) override { return _to DragEventDispatchedToChildProcess(); } \
  NS_IMETHOD UpdateDragEffect(void) override { return _to UpdateDragEffect(); } \
  NS_IMETHOD UpdateDragImage(nsINode *aImage, int32_t aImageX, int32_t aImageY) override { return _to UpdateDragImage(aImage, aImageX, aImageY); } \
  virtual uint32_t GetEffectAllowedForTests(void) override { return _to GetEffectAllowedForTests(); } \
  virtual bool IsSynthesizedForTests(void) override { return _to IsSynthesizedForTests(); } \
  NS_IMETHOD SetDragEndPointForTests(int32_t aScreenX, int32_t aScreenY) override { return _to SetDragEndPointForTests(aScreenX, aScreenY); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIDRAGSESSION(_to) \
  NS_IMETHOD GetCanDrop(bool *aCanDrop) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCanDrop(aCanDrop); } \
  NS_IMETHOD SetCanDrop(bool aCanDrop) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetCanDrop(aCanDrop); } \
  NS_IMETHOD GetOnlyChromeDrop(bool *aOnlyChromeDrop) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOnlyChromeDrop(aOnlyChromeDrop); } \
  NS_IMETHOD SetOnlyChromeDrop(bool aOnlyChromeDrop) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetOnlyChromeDrop(aOnlyChromeDrop); } \
  NS_IMETHOD GetDragAction(uint32_t *aDragAction) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDragAction(aDragAction); } \
  NS_IMETHOD SetDragAction(uint32_t aDragAction) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetDragAction(aDragAction); } \
  NS_IMETHOD GetNumDropItems(uint32_t *aNumDropItems) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNumDropItems(aNumDropItems); } \
  NS_IMETHOD GetSourceDocument(mozilla::dom::Document **aSourceDocument) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSourceDocument(aSourceDocument); } \
  NS_IMETHOD GetSourceNode(nsINode **aSourceNode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSourceNode(aSourceNode); } \
  NS_IMETHOD GetTriggeringPrincipal(nsIPrincipal **aTriggeringPrincipal) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTriggeringPrincipal(aTriggeringPrincipal); } \
  NS_IMETHOD SetTriggeringPrincipal(nsIPrincipal *aTriggeringPrincipal) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetTriggeringPrincipal(aTriggeringPrincipal); } \
  NS_IMETHOD GetCsp(nsIContentSecurityPolicy **aCsp) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCsp(aCsp); } \
  NS_IMETHOD SetCsp(nsIContentSecurityPolicy *aCsp) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetCsp(aCsp); } \
  NS_IMETHOD GetDataTransferXPCOM(mozilla::dom::DataTransfer **aDataTransfer) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDataTransferXPCOM(aDataTransfer); } \
  NS_IMETHOD SetDataTransferXPCOM(mozilla::dom::DataTransfer *aDataTransfer) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetDataTransferXPCOM(aDataTransfer); } \
  virtual mozilla::dom::DataTransfer * GetDataTransfer(void) override; \
  virtual void SetDataTransfer(mozilla::dom::DataTransfer *aDataTransfer) override; \
  NS_IMETHOD GetData(nsITransferable *aTransferable, uint32_t aItemIndex) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetData(aTransferable, aItemIndex); } \
  NS_IMETHOD IsDataFlavorSupported(const char * aDataFlavor, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsDataFlavorSupported(aDataFlavor, _retval); } \
  NS_IMETHOD UserCancelled(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->UserCancelled(); } \
  NS_IMETHOD DragEventDispatchedToChildProcess(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DragEventDispatchedToChildProcess(); } \
  NS_IMETHOD UpdateDragEffect(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->UpdateDragEffect(); } \
  NS_IMETHOD UpdateDragImage(nsINode *aImage, int32_t aImageX, int32_t aImageY) override { return !_to ? NS_ERROR_NULL_POINTER : _to->UpdateDragImage(aImage, aImageX, aImageY); } \
  virtual uint32_t GetEffectAllowedForTests(void) override; \
  virtual bool IsSynthesizedForTests(void) override; \
  NS_IMETHOD SetDragEndPointForTests(int32_t aScreenX, int32_t aScreenY) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetDragEndPointForTests(aScreenX, aScreenY); } 


#endif /* __gen_nsIDragSession_h__ */
