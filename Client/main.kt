import java.io.BufferedReader
import java.io.InputStreamReader
import java.io.PrintWriter
import java.net.Socket

fun main() {
    val host = "127.0.0.1"
    val port = 7878

    try {
        val socket = Socket(host, port)
        println("Connected to server at $host:$port")

        // Потоки получения/отправки данных
        val out = PrintWriter(socket.getOutputStream(), true)
        val input = BufferedReader(InputStreamReader(socket.getInputStream()))

        val message = "Ping server message"
        out.println(message)
        println("Sent to server: $message")

        val response = input.readLine()
        println("Received from server: $response")

        socket.close()

    } catch (e: Exception) {
        e.printStackTrace()
    }
}
