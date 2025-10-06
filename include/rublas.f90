module rublas
  use iso_c_binding
  implicit none

  interface
    ! axpy
    subroutine rublas_saxpy(n, alpha, x, inc_x, y, inc_y) bind(C, name="rublas_saxpy")
      import :: c_int, c_float
      integer(c_int), value :: n, inc_x, inc_y
      real(c_float), value :: alpha
      real(c_float), dimension(*), intent(in) :: x
      real(c_float), dimension(*), intent(inout) :: y
    end subroutine

    subroutine rublas_daxpy(n, alpha, x, inc_x, y, inc_y) bind(C, name="rublas_daxpy")
      import :: c_int, c_double
      integer(c_int), value :: n, inc_x, inc_y
      real(c_double), value :: alpha
      real(c_double), dimension(*), intent(in) :: x
      real(c_double), dimension(*), intent(inout) :: y
    end subroutine

    subroutine rublas_caxpy(n, alpha, x, inc_x, y, inc_y) bind(C, name="rublas_caxpy")
      import :: c_int, c_float_complex
      integer(c_int), value :: n, inc_x, inc_y
      complex(c_float_complex), value :: alpha
      complex(c_float_complex), dimension(*), intent(in) :: x
      complex(c_float_complex), dimension(*), intent(inout) :: y
    end subroutine

    subroutine rublas_zaxpy(n, alpha, x, inc_x, y, inc_y) bind(C, name="rublas_zaxpy")
      import :: c_int, c_double_complex
      integer(c_int), value :: n, inc_x, inc_y
      complex(c_double_complex), value :: alpha
      complex(c_double_complex), dimension(*), intent(in) :: x
      complex(c_double_complex), dimension(*), intent(inout) :: y
    end subroutine

    ! scal
    subroutine rublas_sscal(n, alpha, x, inc_x) bind(C, name="rublas_sscal")
      import :: c_int, c_float
      integer(c_int), value :: n, inc_x
      real(c_float), value :: alpha
      real(c_float), dimension(*), intent(inout) :: x
    end subroutine

    ! dot
    function rublas_sdot(n, x, inc_x, y, inc_y) result(dot) bind(C, name="rublas_sdot")
      import :: c_int, c_float
      integer(c_int), value :: n, inc_x, inc_y
      real(c_float), dimension(*), intent(in) :: x, y
      real(c_float) :: dot
    end function

    function rublas_ddot(n, x, inc_x, y, inc_y) result(dot) bind(C, name="rublas_ddot")
      import :: c_int, c_double
      integer(c_int), value :: n, inc_x, inc_y
      real(c_double), dimension(*), intent(in) :: x, y
      real(c_double) :: dot
    end function
  end interface

  ! axpy
  procedure(rublas_saxpy), pointer :: saxpy => rublas_saxpy
  procedure(rublas_daxpy), pointer :: daxpy => rublas_daxpy
  procedure(rublas_caxpy), pointer :: caxpy => rublas_caxpy
  procedure(rublas_zaxpy), pointer :: zaxpy => rublas_zaxpy

  ! scal
  procedure(rublas_sscal), pointer :: sscal => rublas_sscal

  ! dot
  procedure(rublas_sdot), pointer :: sdot => rublas_sdot
  procedure(rublas_ddot), pointer :: ddot => rublas_ddot

end module rublas
