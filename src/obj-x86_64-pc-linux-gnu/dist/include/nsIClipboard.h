/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsIClipboard.idl
 */

#ifndef __gen_nsIClipboard_h__
#define __gen_nsIClipboard_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsITransferable_h__
#include "nsITransferable.h"
#endif

#ifndef __gen_nsIClipboardOwner_h__
#include "nsIClipboardOwner.h"
#endif

#ifndef __gen_nsTArray_h__
#include "nsTArray.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIArray; /* forward declaration */


/* starting interface:    nsIClipboard */
#define NS_ICLIPBOARD_IID_STR "ceaa0047-647f-4b8e-ad1c-aff9fa62aa51"

#define NS_ICLIPBOARD_IID \
  {0xceaa0047, 0x647f, 0x4b8e, \
    { 0xad, 0x1c, 0xaf, 0xf9, 0xfa, 0x62, 0xaa, 0x51 }}

class NS_NO_VTABLE nsIClipboard : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICLIPBOARD_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIClipboard;

  enum {
    kSelectionClipboard = 0,
    kGlobalClipboard = 1,
    kFindClipboard = 2,
    kSelectionCache = 3
  };

  /* void setData (in nsITransferable aTransferable, in nsIClipboardOwner anOwner, in long aWhichClipboard); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetData(nsITransferable *aTransferable, nsIClipboardOwner *anOwner, int32_t aWhichClipboard) = 0;

  /* void getData (in nsITransferable aTransferable, in long aWhichClipboard); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetData(nsITransferable *aTransferable, int32_t aWhichClipboard) = 0;

  /* void emptyClipboard (in long aWhichClipboard); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD EmptyClipboard(int32_t aWhichClipboard) = 0;

  /* boolean hasDataMatchingFlavors (in Array<ACString> aFlavorList, in long aWhichClipboard); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD HasDataMatchingFlavors(const nsTArray<nsCString >& aFlavorList, int32_t aWhichClipboard, bool *_retval) = 0;

  /* boolean supportsSelectionClipboard (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SupportsSelectionClipboard(bool *_retval) = 0;

  /* boolean supportsFindClipboard (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SupportsFindClipboard(bool *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIClipboard, NS_ICLIPBOARD_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICLIPBOARD \
  NS_IMETHOD SetData(nsITransferable *aTransferable, nsIClipboardOwner *anOwner, int32_t aWhichClipboard) override; \
  NS_IMETHOD GetData(nsITransferable *aTransferable, int32_t aWhichClipboard) override; \
  NS_IMETHOD EmptyClipboard(int32_t aWhichClipboard) override; \
  NS_IMETHOD HasDataMatchingFlavors(const nsTArray<nsCString >& aFlavorList, int32_t aWhichClipboard, bool *_retval) override; \
  NS_IMETHOD SupportsSelectionClipboard(bool *_retval) override; \
  NS_IMETHOD SupportsFindClipboard(bool *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICLIPBOARD \
  nsresult SetData(nsITransferable *aTransferable, nsIClipboardOwner *anOwner, int32_t aWhichClipboard); \
  nsresult GetData(nsITransferable *aTransferable, int32_t aWhichClipboard); \
  nsresult EmptyClipboard(int32_t aWhichClipboard); \
  nsresult HasDataMatchingFlavors(const nsTArray<nsCString >& aFlavorList, int32_t aWhichClipboard, bool *_retval); \
  nsresult SupportsSelectionClipboard(bool *_retval); \
  nsresult SupportsFindClipboard(bool *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICLIPBOARD(_to) \
  NS_IMETHOD SetData(nsITransferable *aTransferable, nsIClipboardOwner *anOwner, int32_t aWhichClipboard) override { return _to SetData(aTransferable, anOwner, aWhichClipboard); } \
  NS_IMETHOD GetData(nsITransferable *aTransferable, int32_t aWhichClipboard) override { return _to GetData(aTransferable, aWhichClipboard); } \
  NS_IMETHOD EmptyClipboard(int32_t aWhichClipboard) override { return _to EmptyClipboard(aWhichClipboard); } \
  NS_IMETHOD HasDataMatchingFlavors(const nsTArray<nsCString >& aFlavorList, int32_t aWhichClipboard, bool *_retval) override { return _to HasDataMatchingFlavors(aFlavorList, aWhichClipboard, _retval); } \
  NS_IMETHOD SupportsSelectionClipboard(bool *_retval) override { return _to SupportsSelectionClipboard(_retval); } \
  NS_IMETHOD SupportsFindClipboard(bool *_retval) override { return _to SupportsFindClipboard(_retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICLIPBOARD(_to) \
  NS_IMETHOD SetData(nsITransferable *aTransferable, nsIClipboardOwner *anOwner, int32_t aWhichClipboard) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetData(aTransferable, anOwner, aWhichClipboard); } \
  NS_IMETHOD GetData(nsITransferable *aTransferable, int32_t aWhichClipboard) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetData(aTransferable, aWhichClipboard); } \
  NS_IMETHOD EmptyClipboard(int32_t aWhichClipboard) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EmptyClipboard(aWhichClipboard); } \
  NS_IMETHOD HasDataMatchingFlavors(const nsTArray<nsCString >& aFlavorList, int32_t aWhichClipboard, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HasDataMatchingFlavors(aFlavorList, aWhichClipboard, _retval); } \
  NS_IMETHOD SupportsSelectionClipboard(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SupportsSelectionClipboard(_retval); } \
  NS_IMETHOD SupportsFindClipboard(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SupportsFindClipboard(_retval); } 



#endif /* __gen_nsIClipboard_h__ */
