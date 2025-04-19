import java.awt.image.BufferedImage;
import java.io.File;
import java.io.IOException;
import java.io.OutputStream;
import java.nio.file.Files;
import java.nio.file.Path;
import javax.imageio.ImageIO;

/**
 * @author Thomas Singer
 */
final class Main {

	public static void main(String[] args) throws IOException {
		final BufferedImage image = ImageIO.read(new File("bold.png"));
		final int width = image.getWidth();
		final int height = image.getHeight();
		assert width % 8 == 0;
		assert height % 8 == 0;

		try (OutputStream os = Files.newOutputStream(Path.of("bitmap.bin"))) {
			for (int row = 0; row < height; row += 8) {
				for (int column = 0; column < width; column += 8) {
					for (int pixelRow = 0; pixelRow < 8; pixelRow++) {
						int value = 0;
						for (int pixelColumn = 0; pixelColumn < 8; pixelColumn++) {
							final int rgb = image.getRGB(column + pixelColumn, row + pixelRow);
							final int gray = gray(rgb);
							final int pixel = gray < 128 ? 1 : 0;
							value = (value << 1) + pixel;
						}
						os.write(value);
					}
				}
			}
		}
	}

	private static int gray(int rgb) {
		int red = (rgb >> 16) & 255;
		int green = (rgb >> 8) & 255;
		int blue = rgb & 255;
		return (red + green + blue) / 3;
	}
}
