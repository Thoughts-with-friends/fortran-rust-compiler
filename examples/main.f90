program main
  implicit none
  integer :: i, a

  a = 0

  do i = 1, 100
    a = a + i
  end do

  print*, a

  stop
end program
