/* -*- Mode: C++; tab-width: 2; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* vim:set ts=2 sw=2 sts=2 et cindent: */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#include "mozilla/dom/TestExampleGenBinding.h"
#include "mozilla/dom/TestExampleThrowingConstructorInterface.h"

namespace mozilla {
namespace dom {


// Only needed for refcounted objects.
NS_IMPL_CYCLE_COLLECTION_WRAPPERCACHE_0(TestExampleThrowingConstructorInterface)
NS_IMPL_CYCLE_COLLECTING_ADDREF(TestExampleThrowingConstructorInterface)
NS_IMPL_CYCLE_COLLECTING_RELEASE(TestExampleThrowingConstructorInterface)
NS_INTERFACE_MAP_BEGIN_CYCLE_COLLECTION(TestExampleThrowingConstructorInterface)
  NS_WRAPPERCACHE_INTERFACE_MAP_ENTRY
  NS_INTERFACE_MAP_ENTRY(nsISupports)
NS_INTERFACE_MAP_END

TestExampleThrowingConstructorInterface::TestExampleThrowingConstructorInterface()
{
    // Add |MOZ_COUNT_CTOR(TestExampleThrowingConstructorInterface);| for a non-refcounted object.
}

TestExampleThrowingConstructorInterface::~TestExampleThrowingConstructorInterface()
{
    // Add |MOZ_COUNT_DTOR(TestExampleThrowingConstructorInterface);| for a non-refcounted object.
}

JSObject*
TestExampleThrowingConstructorInterface::WrapObject(JSContext* aCx, JS::Handle<JSObject*> aGivenProto)
{
  return TestExampleThrowingConstructorInterface_Binding::Wrap(aCx, this, aGivenProto);
}


} // namespace dom
} // namespace mozilla
