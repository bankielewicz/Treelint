using Xunit;
using {{NAMESPACE}};

namespace {{TEST_NAMESPACE}}
{
    public class {{CLASS_NAME}}Tests
    {
        [Fact]
        public void {{METHOD_NAME}}_ValidInput_ReturnsExpectedResult()
        {
            // Arrange
            var sut = new {{CLASS_NAME}}();
            // TODO: Set up test data

            // Act
            var result = sut.{{METHOD_NAME}}(/* parameters */);

            // Assert
            Assert.NotNull(result);
            // TODO: Add specific assertions
        }

        [Fact]
        public void {{METHOD_NAME}}_NullInput_ThrowsArgumentNullException()
        {
            // Arrange
            var sut = new {{CLASS_NAME}}();

            // Act & Assert
            Assert.Throws<ArgumentNullException>(() => sut.{{METHOD_NAME}}(null));
        }

        [Fact]
        public void {{METHOD_NAME}}_InvalidInput_ReturnsError()
        {
            // Arrange
            var sut = new {{CLASS_NAME}}();
            // TODO: Create invalid input

            // Act
            var result = sut.{{METHOD_NAME}}(/* invalid input */);

            // Assert
            Assert.False(result.IsSuccess);
            // TODO: Verify error message
        }
    }
}
