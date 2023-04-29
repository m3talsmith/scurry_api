import 'dart:io';
import 'package:supabase/supabase.dart';

class Database {
  static final SupabaseClient client = _initialize;
  static SupabaseClient get _initialize {
    final apiUrl = Platform.environment['SUPABASE_API_URL'] ?? '';
    final apiKey = Platform.environment['SUPABASE_API_KEY'] ?? '';
    return SupabaseClient(apiUrl, apiKey);
  }
}
