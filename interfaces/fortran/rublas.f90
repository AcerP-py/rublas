module rublas
  use iso_c_binding
  implicit none

  interface

    !!
    !! Level 1 BLAS
    !!

    ! axpy
    subroutine saxpy(n, alpha, x, inc_x, y, inc_y) bind(C, name="axpy_float32")
      import :: c_int, c_float
      integer(c_int), value :: n, inc_x, inc_y
      real(c_float), value :: alpha
      real(c_float), dimension(*), intent(in) :: x
      real(c_float), dimension(*), intent(inout) :: y
    end subroutine

    subroutine daxpy(n, alpha, x, inc_x, y, inc_y) bind(C, name="axpy_float64")
      import :: c_int, c_double
      integer(c_int), value :: n, inc_x, inc_y
      real(c_double), value :: alpha
      real(c_double), dimension(*), intent(in) :: x
      real(c_double), dimension(*), intent(inout) :: y
    end subroutine

    subroutine caxpy(n, alpha, x, inc_x, y, inc_y) bind(C, name="axpy_cfloat32")
      import :: c_int, c_float_complex
      integer(c_int), value :: n, inc_x, inc_y
      complex(c_float_complex), value :: alpha
      complex(c_float_complex), dimension(*), intent(in) :: x
      complex(c_float_complex), dimension(*), intent(inout) :: y
    end subroutine

    subroutine zaxpy(n, alpha, x, inc_x, y, inc_y) bind(C, name="axpy_cfloat64")
      import :: c_int, c_double_complex
      integer(c_int), value :: n, inc_x, inc_y
      complex(c_double_complex), value :: alpha
      complex(c_double_complex), dimension(*), intent(in) :: x
      complex(c_double_complex), dimension(*), intent(inout) :: y
    end subroutine

  end interface

end module rublas
