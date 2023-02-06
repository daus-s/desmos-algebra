import java.io.File;
import java.io.FileNotFoundException;
import java.io.FileWriter;
import java.io.IOException;
import java.util.Scanner;

public class EditFile {
    public static void main(String[] args) {
        String filename = "iasd1844.desmos";
        StringBuilder contents = new StringBuilder();
        try {
            File text = new File(filename);
            Scanner read = new Scanner(text);
            while (read.hasNextLine()) {
                String data = read.nextLine();
                contents.append(data);
            }
            read.close();
        }
        catch (FileNotFoundException e)
        {
            e.printStackTrace();
        }

        String cat = contents.toString();
        cat = cat.replace("\n", "");
        int start = cat.indexOf("[Abstract Syntax]") + 17; // end of this string
        int end = cat.indexOf("[Linearized tree]");
        cat = cat.substring(start, end);

        try
        {
            FileWriter write = new FileWriter(filename);
            write.write(cat);
            write.close();
        }
        catch (IOException e)
        {

            System.out.print(e.getMessage());
        }
    }
}