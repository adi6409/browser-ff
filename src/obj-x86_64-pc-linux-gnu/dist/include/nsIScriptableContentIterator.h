/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/base/nsIScriptableContentIterator.idl
 */

#ifndef __gen_nsIScriptableContentIterator_h__
#define __gen_nsIScriptableContentIterator_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsINode; /* webidl Node */

class nsRange; /* webidl Range */


/* starting interface:    nsIScriptableContentIterator */
#define NS_ISCRIPTABLECONTENTITERATOR_IID_STR "9f25fb2a-265f-44f9-a122-62bbf443239e"

#define NS_ISCRIPTABLECONTENTITERATOR_IID \
  {0x9f25fb2a, 0x265f, 0x44f9, \
    { 0xa1, 0x22, 0x62, 0xbb, 0xf4, 0x43, 0x23, 0x9e }}

class NS_NO_VTABLE nsIScriptableContentIterator : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISCRIPTABLECONTENTITERATOR_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIScriptableContentIterator;

  enum IteratorType : uint8_t {
    NOT_INITIALIZED = 0,
    POST_ORDER_ITERATOR = 1,
    PRE_ORDER_ITERATOR = 2,
    SUBTREE_ITERATOR = 3,
  };

  /* void initWithRootNode (in nsIScriptableContentIterator_IteratorType aType, in Node aRoot); */
  NS_IMETHOD InitWithRootNode(nsIScriptableContentIterator::IteratorType aType, nsINode *aRoot) = 0;

  /* void initWithRange (in nsIScriptableContentIterator_IteratorType aType, in Range aRange); */
  NS_IMETHOD InitWithRange(nsIScriptableContentIterator::IteratorType aType, nsRange *aRange) = 0;

  /* void initWithPositions (in nsIScriptableContentIterator_IteratorType aType, in Node aStartContainer, in unsigned long aStartOffset, in Node aEndContainer, in unsigned long aEndOffset); */
  NS_IMETHOD InitWithPositions(nsIScriptableContentIterator::IteratorType aType, nsINode *aStartContainer, uint32_t aStartOffset, nsINode *aEndContainer, uint32_t aEndOffset) = 0;

  /* void first (); */
  NS_IMETHOD First(void) = 0;

  /* void last (); */
  NS_IMETHOD Last(void) = 0;

  /* void next (); */
  NS_IMETHOD Next(void) = 0;

  /* void prev (); */
  NS_IMETHOD Prev(void) = 0;

  /* readonly attribute Node currentNode; */
  NS_IMETHOD GetCurrentNode(nsINode **aCurrentNode) = 0;

  /* readonly attribute bool isDone; */
  NS_IMETHOD GetIsDone(bool *aIsDone) = 0;

  /* void positionAt (in Node aNode); */
  NS_IMETHOD PositionAt(nsINode *aNode) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIScriptableContentIterator, NS_ISCRIPTABLECONTENTITERATOR_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISCRIPTABLECONTENTITERATOR \
  NS_IMETHOD InitWithRootNode(nsIScriptableContentIterator::IteratorType aType, nsINode *aRoot) override; \
  NS_IMETHOD InitWithRange(nsIScriptableContentIterator::IteratorType aType, nsRange *aRange) override; \
  NS_IMETHOD InitWithPositions(nsIScriptableContentIterator::IteratorType aType, nsINode *aStartContainer, uint32_t aStartOffset, nsINode *aEndContainer, uint32_t aEndOffset) override; \
  NS_IMETHOD First(void) override; \
  NS_IMETHOD Last(void) override; \
  NS_IMETHOD Next(void) override; \
  NS_IMETHOD Prev(void) override; \
  NS_IMETHOD GetCurrentNode(nsINode **aCurrentNode) override; \
  NS_IMETHOD GetIsDone(bool *aIsDone) override; \
  NS_IMETHOD PositionAt(nsINode *aNode) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISCRIPTABLECONTENTITERATOR \
  nsresult InitWithRootNode(nsIScriptableContentIterator::IteratorType aType, nsINode *aRoot); \
  nsresult InitWithRange(nsIScriptableContentIterator::IteratorType aType, nsRange *aRange); \
  nsresult InitWithPositions(nsIScriptableContentIterator::IteratorType aType, nsINode *aStartContainer, uint32_t aStartOffset, nsINode *aEndContainer, uint32_t aEndOffset); \
  nsresult First(void); \
  nsresult Last(void); \
  nsresult Next(void); \
  nsresult Prev(void); \
  nsresult GetCurrentNode(nsINode **aCurrentNode); \
  nsresult GetIsDone(bool *aIsDone); \
  nsresult PositionAt(nsINode *aNode); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISCRIPTABLECONTENTITERATOR(_to) \
  NS_IMETHOD InitWithRootNode(nsIScriptableContentIterator::IteratorType aType, nsINode *aRoot) override { return _to InitWithRootNode(aType, aRoot); } \
  NS_IMETHOD InitWithRange(nsIScriptableContentIterator::IteratorType aType, nsRange *aRange) override { return _to InitWithRange(aType, aRange); } \
  NS_IMETHOD InitWithPositions(nsIScriptableContentIterator::IteratorType aType, nsINode *aStartContainer, uint32_t aStartOffset, nsINode *aEndContainer, uint32_t aEndOffset) override { return _to InitWithPositions(aType, aStartContainer, aStartOffset, aEndContainer, aEndOffset); } \
  NS_IMETHOD First(void) override { return _to First(); } \
  NS_IMETHOD Last(void) override { return _to Last(); } \
  NS_IMETHOD Next(void) override { return _to Next(); } \
  NS_IMETHOD Prev(void) override { return _to Prev(); } \
  NS_IMETHOD GetCurrentNode(nsINode **aCurrentNode) override { return _to GetCurrentNode(aCurrentNode); } \
  NS_IMETHOD GetIsDone(bool *aIsDone) override { return _to GetIsDone(aIsDone); } \
  NS_IMETHOD PositionAt(nsINode *aNode) override { return _to PositionAt(aNode); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISCRIPTABLECONTENTITERATOR(_to) \
  NS_IMETHOD InitWithRootNode(nsIScriptableContentIterator::IteratorType aType, nsINode *aRoot) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InitWithRootNode(aType, aRoot); } \
  NS_IMETHOD InitWithRange(nsIScriptableContentIterator::IteratorType aType, nsRange *aRange) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InitWithRange(aType, aRange); } \
  NS_IMETHOD InitWithPositions(nsIScriptableContentIterator::IteratorType aType, nsINode *aStartContainer, uint32_t aStartOffset, nsINode *aEndContainer, uint32_t aEndOffset) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InitWithPositions(aType, aStartContainer, aStartOffset, aEndContainer, aEndOffset); } \
  NS_IMETHOD First(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->First(); } \
  NS_IMETHOD Last(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Last(); } \
  NS_IMETHOD Next(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Next(); } \
  NS_IMETHOD Prev(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Prev(); } \
  NS_IMETHOD GetCurrentNode(nsINode **aCurrentNode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCurrentNode(aCurrentNode); } \
  NS_IMETHOD GetIsDone(bool *aIsDone) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsDone(aIsDone); } \
  NS_IMETHOD PositionAt(nsINode *aNode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->PositionAt(aNode); } 

#define SCRIPTABLE_CONTENT_ITERATOR_CID \
  { 0xf68037ec, 0x2790, 0x44c5, \
    { 0x8e, 0x5f, 0xdf, 0x5d, 0xa5, 0x8b, 0x93, 0xa7 } }
#define SCRIPTABLE_CONTENT_ITERATOR_CONTRACTID \
  "@mozilla.org/scriptable-content-iterator;1"

#endif /* __gen_nsIScriptableContentIterator_h__ */
