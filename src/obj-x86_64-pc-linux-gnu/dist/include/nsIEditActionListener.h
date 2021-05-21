/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/editor/nsIEditActionListener.idl
 */

#ifndef __gen_nsIEditActionListener_h__
#define __gen_nsIEditActionListener_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_domstubs_h__
#include "domstubs.h"
#endif

#ifndef __gen_nsTArray_h__
#include "nsTArray.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
namespace mozilla {
namespace dom {
class CharacterData; /* webidl CharacterData */
} // namespace dom
} // namespace mozilla

class nsINode; /* webidl Node */

class nsRange; /* webidl Range */


/* starting interface:    nsIEditActionListener */
#define NS_IEDITACTIONLISTENER_IID_STR "b22907b1-ee93-11d2-8d50-000064657374"

#define NS_IEDITACTIONLISTENER_IID \
  {0xb22907b1, 0xee93, 0x11d2, \
    { 0x8d, 0x50, 0x00, 0x00, 0x64, 0x65, 0x73, 0x74 }}

class NS_NO_VTABLE nsIEditActionListener : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IEDITACTIONLISTENER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIEditActionListener;

  /* void DidDeleteNode (in Node aChild, in nsresult aResult); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD DidDeleteNode(nsINode *aChild, nsresult aResult) = 0;

  /* void DidJoinNodes (in Node aLeftNode, in Node aRightNode, in Node aParent, in nsresult aResult); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD DidJoinNodes(nsINode *aLeftNode, nsINode *aRightNode, nsINode *aParent, nsresult aResult) = 0;

  /* void DidInsertText (in CharacterData aTextNode, in long aOffset, in AString aString, in nsresult aResult); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD DidInsertText(mozilla::dom::CharacterData *aTextNode, int32_t aOffset, const nsAString& aString, nsresult aResult) = 0;

  /* void WillDeleteText (in CharacterData aTextNode, in long aOffset, in long aLength); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD WillDeleteText(mozilla::dom::CharacterData *aTextNode, int32_t aOffset, int32_t aLength) = 0;

  /* void WillDeleteRanges (in Array<Range> aRangesToDelete); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD WillDeleteRanges(const nsTArray<RefPtr<nsRange>>& aRangesToDelete) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIEditActionListener, NS_IEDITACTIONLISTENER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIEDITACTIONLISTENER \
  NS_IMETHOD DidDeleteNode(nsINode *aChild, nsresult aResult) override; \
  NS_IMETHOD DidJoinNodes(nsINode *aLeftNode, nsINode *aRightNode, nsINode *aParent, nsresult aResult) override; \
  NS_IMETHOD DidInsertText(mozilla::dom::CharacterData *aTextNode, int32_t aOffset, const nsAString& aString, nsresult aResult) override; \
  NS_IMETHOD WillDeleteText(mozilla::dom::CharacterData *aTextNode, int32_t aOffset, int32_t aLength) override; \
  NS_IMETHOD WillDeleteRanges(const nsTArray<RefPtr<nsRange>>& aRangesToDelete) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIEDITACTIONLISTENER \
  nsresult DidDeleteNode(nsINode *aChild, nsresult aResult); \
  nsresult DidJoinNodes(nsINode *aLeftNode, nsINode *aRightNode, nsINode *aParent, nsresult aResult); \
  nsresult DidInsertText(mozilla::dom::CharacterData *aTextNode, int32_t aOffset, const nsAString& aString, nsresult aResult); \
  nsresult WillDeleteText(mozilla::dom::CharacterData *aTextNode, int32_t aOffset, int32_t aLength); \
  nsresult WillDeleteRanges(const nsTArray<RefPtr<nsRange>>& aRangesToDelete); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIEDITACTIONLISTENER(_to) \
  NS_IMETHOD DidDeleteNode(nsINode *aChild, nsresult aResult) override { return _to DidDeleteNode(aChild, aResult); } \
  NS_IMETHOD DidJoinNodes(nsINode *aLeftNode, nsINode *aRightNode, nsINode *aParent, nsresult aResult) override { return _to DidJoinNodes(aLeftNode, aRightNode, aParent, aResult); } \
  NS_IMETHOD DidInsertText(mozilla::dom::CharacterData *aTextNode, int32_t aOffset, const nsAString& aString, nsresult aResult) override { return _to DidInsertText(aTextNode, aOffset, aString, aResult); } \
  NS_IMETHOD WillDeleteText(mozilla::dom::CharacterData *aTextNode, int32_t aOffset, int32_t aLength) override { return _to WillDeleteText(aTextNode, aOffset, aLength); } \
  NS_IMETHOD WillDeleteRanges(const nsTArray<RefPtr<nsRange>>& aRangesToDelete) override { return _to WillDeleteRanges(aRangesToDelete); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIEDITACTIONLISTENER(_to) \
  NS_IMETHOD DidDeleteNode(nsINode *aChild, nsresult aResult) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DidDeleteNode(aChild, aResult); } \
  NS_IMETHOD DidJoinNodes(nsINode *aLeftNode, nsINode *aRightNode, nsINode *aParent, nsresult aResult) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DidJoinNodes(aLeftNode, aRightNode, aParent, aResult); } \
  NS_IMETHOD DidInsertText(mozilla::dom::CharacterData *aTextNode, int32_t aOffset, const nsAString& aString, nsresult aResult) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DidInsertText(aTextNode, aOffset, aString, aResult); } \
  NS_IMETHOD WillDeleteText(mozilla::dom::CharacterData *aTextNode, int32_t aOffset, int32_t aLength) override { return !_to ? NS_ERROR_NULL_POINTER : _to->WillDeleteText(aTextNode, aOffset, aLength); } \
  NS_IMETHOD WillDeleteRanges(const nsTArray<RefPtr<nsRange>>& aRangesToDelete) override { return !_to ? NS_ERROR_NULL_POINTER : _to->WillDeleteRanges(aRangesToDelete); } 


#endif /* __gen_nsIEditActionListener_h__ */
