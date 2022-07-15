package projava;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.*;

@SuppressWarnings("NoAsciiCharacters")
class OlympicTest {
    @Test
    void 近代オリンピック開始以前(){
        assertFalse(new Olympic().isSummerOlympicYear(1888),"1888年");
        assertFalse(new Olympic().isSummerOlympicYear(1892),"1892年");
        assertFalse(new Olympic().isSummerOlympicYear(1895,"1895年");
        //初回開催
        assertTrue(new Olympic().isSummerOlympicYear(1896),"1896年");
    }

    @Test
    void 四年周期の一般的な開催年(){
        int[] years = {1900,1920,1936,2000};
        for(int year:years){
            assertTrue(new Olymic().isSummerOlympicYear(year),year + "年");
        }
    }
}
