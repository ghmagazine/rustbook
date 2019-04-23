import java.util.List;
import java.util.ArrayList;

public class Overload {
    public static void main(String[] args) {
        List<String> list = new ArrayList<String>();
        // オーバーロードだとListとして扱われる
        assert "List".equals(Overload.call(list));
    }

    // Listに対して定義する
    static String call(List<String> list) {
        return "List";
    }

    // ArrayListに対して定義する
    static String call(ArrayList<String> list) {
        return "ArrayList";
    }
}
