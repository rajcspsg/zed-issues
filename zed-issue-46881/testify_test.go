package example_test

import (
	"testing"

	"github.com/stretchr/testify/suite"
)

type Suite struct {
	suite.Suite
}

func TestSuite(t *testing.T) {
	suite.Run(t, new(Suite))
}

func (s *Suite) TestA() {
	s.Require().True(true)
}

func (s *Suite) TestB() {
	s.Run("B.1", func() {
		s.Require().True(true)
	})

	s.Run("B.2", func() {
		s.Require().True(false)
	})
}
