use pin_project::pin_project;
#[pin(__private(project = Proj, project_ref = ProjRef, project_replace = ProjOwn))]
enum Enum<T, U> {
    Struct { #[pin] pinned: T, unpinned: U },
    Tuple(#[pin] T, U),
    Unit,
}
#[allow(dead_code)]
#[allow(single_use_lifetimes)]
#[allow(clippy::mut_mut)]
#[allow(clippy::type_repetition_in_bounds)]
enum Proj<'pin, T, U>
where
    Enum<T, U>: 'pin,
{
    Struct {
        pinned: ::pin_project::__private::Pin<&'pin mut (T)>,
        unpinned: &'pin mut (U),
    },
    Tuple(::pin_project::__private::Pin<&'pin mut (T)>, &'pin mut (U)),
    Unit,
}
#[allow(dead_code)]
#[allow(single_use_lifetimes)]
#[allow(clippy::type_repetition_in_bounds)]
enum ProjRef<'pin, T, U>
where
    Enum<T, U>: 'pin,
{
    Struct { pinned: ::pin_project::__private::Pin<&'pin (T)>, unpinned: &'pin (U) },
    Tuple(::pin_project::__private::Pin<&'pin (T)>, &'pin (U)),
    Unit,
}
#[allow(dead_code)]
#[allow(single_use_lifetimes)]
#[allow(unreachable_pub)]
enum ProjOwn<T, U> {
    Struct { pinned: ::pin_project::__private::PhantomData<T>, unpinned: U },
    Tuple(::pin_project::__private::PhantomData<T>, U),
    Unit,
}
#[doc(hidden)]
#[allow(non_upper_case_globals)]
#[allow(single_use_lifetimes)]
#[allow(clippy::used_underscore_binding)]
const _: () = {
    impl<T, U> Enum<T, U> {
        #[allow(dead_code)]
        fn project<'pin>(
            self: ::pin_project::__private::Pin<&'pin mut Self>,
        ) -> Proj<'pin, T, U> {
            unsafe {
                match self.get_unchecked_mut() {
                    Enum::Struct { pinned, unpinned } => {
                        Proj::Struct {
                            pinned: ::pin_project::__private::Pin::new_unchecked(pinned),
                            unpinned,
                        }
                    }
                    Enum::Tuple(_0, _1) => {
                        Proj::Tuple(::pin_project::__private::Pin::new_unchecked(_0), _1)
                    }
                    Enum::Unit => Proj::Unit,
                }
            }
        }
        #[allow(dead_code)]
        fn project_ref<'pin>(
            self: ::pin_project::__private::Pin<&'pin Self>,
        ) -> ProjRef<'pin, T, U> {
            unsafe {
                match self.get_ref() {
                    Enum::Struct { pinned, unpinned } => {
                        ProjRef::Struct {
                            pinned: ::pin_project::__private::Pin::new_unchecked(pinned),
                            unpinned,
                        }
                    }
                    Enum::Tuple(_0, _1) => {
                        ProjRef::Tuple(
                            ::pin_project::__private::Pin::new_unchecked(_0),
                            _1,
                        )
                    }
                    Enum::Unit => ProjRef::Unit,
                }
            }
        }
        #[allow(dead_code)]
        fn project_replace(
            self: ::pin_project::__private::Pin<&mut Self>,
            __replacement: Self,
        ) -> ProjOwn<T, U> {
            unsafe {
                let __self_ptr: *mut Self = self.get_unchecked_mut();
                match &mut *__self_ptr {
                    Enum::Struct { pinned, unpinned } => {
                        let __result = ProjOwn::Struct {
                            pinned: ::pin_project::__private::PhantomData,
                            unpinned: ::pin_project::__private::ptr::read(unpinned),
                        };
                        let __guard = ::pin_project::__private::UnsafeOverwriteGuard {
                            target: __self_ptr,
                            value: ::pin_project::__private::ManuallyDrop::new(
                                __replacement,
                            ),
                        };
                        {
                            let __guard = ::pin_project::__private::UnsafeDropInPlaceGuard(
                                pinned,
                            );
                        }
                        __result
                    }
                    Enum::Tuple(_0, _1) => {
                        let __result = ProjOwn::Tuple(
                            ::pin_project::__private::PhantomData,
                            ::pin_project::__private::ptr::read(_1),
                        );
                        let __guard = ::pin_project::__private::UnsafeOverwriteGuard {
                            target: __self_ptr,
                            value: ::pin_project::__private::ManuallyDrop::new(
                                __replacement,
                            ),
                        };
                        {
                            let __guard = ::pin_project::__private::UnsafeDropInPlaceGuard(
                                _0,
                            );
                        }
                        __result
                    }
                    Enum::Unit => {
                        let __result = ProjOwn::Unit;
                        let __guard = ::pin_project::__private::UnsafeOverwriteGuard {
                            target: __self_ptr,
                            value: ::pin_project::__private::ManuallyDrop::new(
                                __replacement,
                            ),
                        };
                        {}
                        __result
                    }
                }
            }
        }
    }
    struct __Enum<'pin, T, U> {
        __pin_project_use_generics: ::pin_project::__private::AlwaysUnpin<
            'pin,
            (
                ::pin_project::__private::PhantomData<T>,
                ::pin_project::__private::PhantomData<U>,
            ),
        >,
        __field0: T,
        __field1: T,
    }
    impl<'pin, T, U> ::pin_project::__private::Unpin for Enum<T, U>
    where
        __Enum<'pin, T, U>: ::pin_project::__private::Unpin,
    {}
    unsafe impl<T, U> ::pin_project::UnsafeUnpin for Enum<T, U> {}
    trait EnumMustNotImplDrop {}
    #[allow(clippy::drop_bounds, drop_bounds)]
    impl<T: ::pin_project::__private::Drop> EnumMustNotImplDrop for T {}
    impl<T, U> EnumMustNotImplDrop for Enum<T, U> {}
    impl<T, U> ::pin_project::__private::PinnedDrop for Enum<T, U> {
        unsafe fn drop(self: ::pin_project::__private::Pin<&mut Self>) {}
    }
};
fn main() {}
