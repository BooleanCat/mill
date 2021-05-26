Describe 'acceptance'
  Describe '--help'
    It 'prints help text'
      When run target/release/mill -h
      The status should be success
      The output should include 'USAGE'
    End

    It 'prints help text when invoked with an unknown subcommand'
      When run target/release/mill nope
      The status should be failure
      The error should include 'USAGE'
    End
  End

  Describe 'state'
    It 'is not implemented'
      When run target/release/mill state foo
      The status should be failure
      The error should include 'not implemented'
    End
  End

  Describe 'create'
    It 'is not implemented'
      When run target/release/mill create foo bar
      The status should be failure
      The error should include 'not implemented'
    End
  End

  Describe 'start'
    It 'is not implemented'
      When run target/release/mill start foo
      The status should be failure
      The error should include 'not implemented'
    End
  End

  Describe 'kill'
    It 'is not implemented'
      When run target/release/mill kill foo SIGTERM
      The status should be failure
      The error should include 'not implemented'
    End

    It 'has an option arg "signal"'
      When run target/release/mill kill foo
      The status should be failure
      The error should include 'not implemented'
    End
  End

  Describe 'delete'
    It 'is not implemented'
      When run target/release/mill delete foo
      The status should be failure
      The error should include 'not implemented'
    End
  End
End